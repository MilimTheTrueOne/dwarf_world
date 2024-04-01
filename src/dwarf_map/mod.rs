use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use self::chunk::{ChunkBundle, ChunkCache, ChunkCord, ChunkData, CHUNK_SIZE};

pub mod chunk;
pub mod tile_atlas;
mod visibility;

pub struct DwarfMapPlugin;

impl Plugin for DwarfMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentMapLayer>()
            .add_plugins(visibility::LayerVisibilityPlugin)
            .add_plugins(chunk::ChunkRenderPlugin)
            .add_systems(Startup, spawn_chunk)
            .add_plugins(ResourceInspectorPlugin::<CurrentMapLayer>::default());
    }
}

pub fn spawn_chunk(mut commands: Commands, mut cache: ResMut<ChunkCache>) {
    for x in 0..2 {
        for y in 0..1 {
            for z in 0..1 {
                let cord = ChunkCord(UVec3::new(x, y, z));
                let e = commands
                    .spawn(ChunkBundle {
                        chunk: ChunkData::random(),
                        transform: Transform::from_xyz(
                            (x as usize * CHUNK_SIZE) as f32,
                            (y as usize * CHUNK_SIZE) as f32,
                            (z as usize * CHUNK_SIZE) as f32,
                        ),
                        cord,
                        ..default()
                    })
                    .id();

                cache.insert(cord.0, e);
            }
        }
    }
}

#[derive(Debug, Resource, Reflect, Deref, DerefMut)]
pub struct CurrentMapLayer(pub usize);

impl Default for CurrentMapLayer {
    fn default() -> Self {
        Self(8)
    }
}

pub mod dwarf_map_flags {
    use bevy::{ecs::component::Component, prelude::Deref};

    #[derive(Debug, Component, Deref)]
    pub struct WallFloorMesh(pub usize);

    #[derive(Debug, Component, Deref)]
    pub struct CeilingMesh(pub usize);
}
