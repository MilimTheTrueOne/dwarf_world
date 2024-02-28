use bevy::prelude::*;

mod chunk;
mod temp_mesh;
mod visibility;

pub struct DwarfMapPlugin;

impl Plugin for DwarfMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Chunk>()
            .init_resource::<CurrentMapLayer>()
            .add_plugins(visibility::LayerVisibilityPlugin)
            .add_systems(Startup, spawn_layers);
    }
}

#[derive(Debug, Resource)]
pub struct Chunk {
    pub tiles: [[[bool; 16]; 16]; 16],
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            tiles: rand::random(),
            //tiles: [[[true; 16]; 16]; 16],
        }
    }
}

fn spawn_layers(
    mut commands: Commands,
    chunk: Res<Chunk>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // material
    let material: Handle<StandardMaterial> = materials.add(Color::rgb(0.8, 0.7, 0.6));

    let mut layer_transform: Transform = Transform::from_xyz(0.0, 0.0, 0.0);

    // generate layers
    for (layer, tiles) in chunk.tiles.windows(3).enumerate() {
        let layer_entity = commands
            .spawn(SpatialBundle::from_transform(layer_transform))
            .insert(Layer)
            .id();

        let (wall_floor_mesh, ceiling_mesh) = chunk::generate_mesh(&tiles[1], &tiles[2], &tiles[0]);

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(wall_floor_mesh.into_mesh()),
                material: material.clone(),
                ..Default::default()
            })
            .insert(dwarf_map_flags::WallMesh(layer))
            .set_parent(layer_entity);

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(ceiling_mesh.into_mesh()),
                material: material.clone(),
                ..Default::default()
            })
            .insert(dwarf_map_flags::CeilingMesh(layer))
            .set_parent(layer_entity);

        layer_transform.translation.y += 1.0;
    }
}

#[derive(Debug, Component)]
pub struct Layer;

#[derive(Debug, Resource, Deref)]
pub struct CurrentMapLayer(pub usize);

impl Default for CurrentMapLayer {
    fn default() -> Self {
        Self(8)
    }
}

pub mod dwarf_map_flags {
    use bevy::{ecs::component::Component, prelude::Deref};

    #[derive(Debug, Component, Deref)]
    pub struct WallMesh(pub usize);

    #[derive(Debug, Component, Deref)]
    pub struct CeilingMesh(pub usize);

    #[derive(Debug, Component, Deref)]
    pub struct FloorMesh(pub usize);
}
