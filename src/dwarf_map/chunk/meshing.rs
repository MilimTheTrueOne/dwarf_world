use super::temp_mesh::TempMesh;

use super::*;

const MAX: u32 = CHUNK_SIZE as u32 - 1;

pub trait MeshLayer {
    fn get_tile(&self, pos: UVec2) -> TileVisibility;
}

#[derive(Debug, Clone, Copy, Default, Reflect)]
pub enum TileVisibility {
    #[default]
    Empty,
    Solid,
    Transparent,
}

impl TileVisibility {
    pub fn visible(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Empty, Empty) => false,
            (Empty, Solid) => false,
            (Empty, Transparent) => false,
            (Solid, Empty) => true,
            (Solid, Solid) => false,
            (Solid, Transparent) => true,
            (Transparent, Empty) => true,
            (Transparent, Solid) => false,
            (Transparent, Transparent) => false,
        }
    }
}
use TileVisibility::*;

impl MeshLayer for [[Tile; CHUNK_SIZE]; CHUNK_SIZE] {
    fn get_tile(&self, pos: UVec2) -> TileVisibility {
        self[pos.x as usize][pos.y as usize].visibility
    }
}

pub struct EmptyLayer;

impl MeshLayer for EmptyLayer {
    fn get_tile(&self, _: UVec2) -> TileVisibility {
        TileVisibility::Empty
    }
}

/// turn any type that implements [`MeshLayer`] into a mesh, given the layer above and below it
/// return the FloorWallMesh and the CeilingMesh
pub fn generate_mesh(
    layer: &[[Tile; CHUNK_SIZE]; CHUNK_SIZE],
    above: &impl MeshLayer,
    below: &impl MeshLayer,
) -> (Mesh, Mesh) {
    let get_neighbors = |pos: UVec2| {
        let x = pos.x;
        let y = pos.y;

        let (left, right) = match x {
            0 => (Empty, layer.get_tile(UVec2::new(x + 1, y))),
            MAX => (layer.get_tile(UVec2::new(x - 1, y)), Empty),
            _ => (
                layer.get_tile(UVec2::new(x - 1, y)),
                layer.get_tile(UVec2::new(x + 1, y)),
            ),
        };

        let (front, back) = match y {
            0 => (Empty, layer.get_tile(UVec2::new(x, y + 1))),
            MAX => (layer.get_tile(UVec2::new(x, y - 1)), Empty),
            _ => (
                layer.get_tile(UVec2::new(x, y - 1)),
                layer.get_tile(UVec2::new(x, y + 1)),
            ),
        };

        [
            above.get_tile(pos),
            below.get_tile(pos),
            right,
            left,
            front,
            back,
        ]
    };

    let mut floor_wall_mesh = TempMesh::new();
    let mut ceiling_mesh = TempMesh::new();

    for x in 0..(CHUNK_SIZE as u32) {
        for z in 0..(CHUNK_SIZE as u32) {
            let vis = layer[x as usize][z as usize].visibility;
            let neighbors = get_neighbors(UVec2::new(x, z));

            let offset = Vec3::new(x as f32, 0.0, z as f32);

            if vis.visible(&neighbors[0]) {
                data::cube::add_ceiling(&mut floor_wall_mesh, &offset)
            } else if vis.visible(&Empty) {
                data::cube::add_ceiling(&mut ceiling_mesh, &offset)
            }

            if vis.visible(&neighbors[1]) {
                data::cube::add_bottom(&mut floor_wall_mesh, &offset)
            }

            if vis.visible(&neighbors[2]) {
                data::cube::add_right(&mut floor_wall_mesh, &offset);
            }

            if vis.visible(&neighbors[3]) {
                data::cube::add_left(&mut floor_wall_mesh, &offset);
            }

            if vis.visible(&neighbors[4]) {
                data::cube::add_front(&mut floor_wall_mesh, &offset);
            }

            if vis.visible(&neighbors[5]) {
                data::cube::add_back(&mut floor_wall_mesh, &offset);
            }
        }
    }

    (floor_wall_mesh.into_mesh(), ceiling_mesh.into_mesh())
}
