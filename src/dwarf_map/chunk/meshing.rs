use super::temp_mesh::TempMesh;

use super::*;

const MAX: u32 = CHUNK_SIZE as u32 - 1;

pub trait MeshLayer {
    fn get_tile(&self, pos: UVec2) -> &Tile;
}

impl MeshLayer for [[Tile; CHUNK_SIZE]; CHUNK_SIZE] {
    fn get_tile(&self, pos: UVec2) -> &Tile {
        &self[pos.x as usize][pos.y as usize]
    }
}

pub struct EmptyLayer;

impl MeshLayer for EmptyLayer {
    fn get_tile(&self, _: UVec2) -> &Tile {
        &false
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
            0 => (false, *layer.get_tile(UVec2::new(x + 1, y))),
            MAX => (*layer.get_tile(UVec2::new(x - 1, y)), false),
            _ => (
                *layer.get_tile(UVec2::new(x - 1, y)),
                *layer.get_tile(UVec2::new(x + 1, y)),
            ),
        };

        let (front, back) = match y {
            0 => (false, *layer.get_tile(UVec2::new(x, y + 1))),
            MAX => (*layer.get_tile(UVec2::new(x, y - 1)), false),
            _ => (
                *layer.get_tile(UVec2::new(x, y - 1)),
                *layer.get_tile(UVec2::new(x, y + 1)),
            ),
        };

        [
            *above.get_tile(pos),
            *below.get_tile(pos),
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
            let tile = layer[x as usize][z as usize];
            let neighbors = get_neighbors(UVec2::new(x, z));

            let offset = Vec3::new(x as f32, 0.0, z as f32);

            if tile {
                data::cube::add_ceiling(&mut ceiling_mesh, &offset);

                if !neighbors[1] {
                    data::cube::add_bottom(&mut floor_wall_mesh, &offset);
                }

                if !neighbors[2] {
                    data::cube::add_right(&mut floor_wall_mesh, &offset);
                }

                if !neighbors[3] {
                    data::cube::add_left(&mut floor_wall_mesh, &offset);
                }

                if !neighbors[4] {
                    data::cube::add_front(&mut floor_wall_mesh, &offset)
                }

                if !neighbors[5] {
                    data::cube::add_back(&mut floor_wall_mesh, &offset)
                }
            } else if neighbors[1] {
                data::cube::add_floor(&mut floor_wall_mesh, &offset)
            }
        }
    }

    (floor_wall_mesh.into_mesh(), ceiling_mesh.into_mesh())
}
