use super::temp_mesh::TempMesh;

use super::*;

#[derive(Debug, Clone, Copy, Default, Reflect)]
pub enum TileVisibility {
    #[default]
    Empty,
    Solid,
}

impl TileVisibility {
    pub fn visible(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Empty, Empty) => false,
            (Empty, Solid) => false,
            (Solid, Empty) => true,
            (Solid, Solid) => false,
        }
    }
}
use TileVisibility::*;

/// turn any type that implements [`MeshLayer`] into a mesh, given the layer above and below it
/// return the FloorWallMesh and the CeilingMesh
pub fn generate_mesh(
    layer: &[[Tile; CHUNK_SIZE]; CHUNK_SIZE],
    get_vis: impl Fn(UVec2, usize) -> [TileVisibility; 6],
    layer_index: usize,
    atlas: &Res<crate::dwarf_map::tile_atlas::TileAtlas>,
) -> (Mesh, Mesh) {
    let mut floor_wall_mesh = TempMesh::new();
    let mut ceiling_mesh = TempMesh::new();

    for x in 0..(CHUNK_SIZE as u32) {
        for z in 0..(CHUNK_SIZE as u32) {
            let vis = layer[x as usize][z as usize].visibility;
            let index = layer[x as usize][z as usize].index;

            let neighbors = get_vis(UVec2::new(x, z), layer_index);

            let offset = Vec3::new(x as f32, 0.0, z as f32);

            if vis.visible(&neighbors[0]) {
                data::cube::add_ceiling(&mut floor_wall_mesh, &offset, atlas.get_uvs(index))
            } else if vis.visible(&Empty) {
                data::cube::add_ceiling(&mut ceiling_mesh, &offset, atlas.get_uvs(index))
            }

            if vis.visible(&neighbors[1]) {
                data::cube::add_bottom(&mut floor_wall_mesh, &offset, atlas.get_uvs(index))
            }

            if vis.visible(&neighbors[2]) {
                data::cube::add_right(&mut floor_wall_mesh, &offset, atlas.get_uvs(index));
            }

            if vis.visible(&neighbors[3]) {
                data::cube::add_left(&mut floor_wall_mesh, &offset, atlas.get_uvs(index));
            }

            if vis.visible(&neighbors[4]) {
                data::cube::add_front(&mut floor_wall_mesh, &offset, atlas.get_uvs(index));
            }

            if vis.visible(&neighbors[5]) {
                data::cube::add_back(&mut floor_wall_mesh, &offset, atlas.get_uvs(index));
            }
        }
    }

    (floor_wall_mesh.into_mesh(), ceiling_mesh.into_mesh())
}
