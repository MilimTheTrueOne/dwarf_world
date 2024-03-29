use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use self::chunk::{ChunkBundle, ChunkData};

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

pub fn spawn_chunk(mut commands: Commands) {
    commands.spawn(ChunkBundle {
        chunk: ChunkData::random(),
        ..Default::default()
    });
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
