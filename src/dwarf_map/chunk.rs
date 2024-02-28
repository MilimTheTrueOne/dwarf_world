use bevy::prelude::*;

pub mod chunk_to_mesh;
pub mod data;
pub use chunk_to_mesh::*;

pub const CHUNK_SIZE: usize = 16;

pub type Tile = bool;

#[derive(Default, Debug, Clone)]
pub struct ChunkData {
    /// this might be better of being moved to another type?
    tiles: [[[Tile; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl ChunkData {
    pub fn random() -> Self {
        Self {
            tiles: rand::random(),
        }
    }

    pub fn get_tile_local(&self, pos: UVec3) -> Tile {
        self.tiles[pos.x as usize][pos.y as usize][pos.z as usize]
    }

    pub fn set_tile_local(&mut self, pos: UVec3, value: Tile) {
        self.tiles[pos.x as usize][pos.y as usize][pos.z as usize] = value;
    }
}

#[derive(Component, Default)]
pub struct ChunkComponent;

#[derive(Bundle, Default)]
pub struct ChunkBundle {
    pub chunk: ChunkComponent,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub global_transform: GlobalTransform,
    pub transform: Transform,
}

impl ChunkBundle {
    pub fn from_transform(transform: Transform) -> Self {
        Self {
            transform,
            ..Default::default()
        }
    }
}
