use bevy::{ecs::system::SystemParam, prelude::*, utils::HashMap};
use rand::{distributions::Standard, prelude::*};

pub mod data;
pub mod meshing;
mod temp_mesh;
pub use meshing::*;

use super::dwarf_map_flags;

pub const CHUNK_SIZE: usize = 16;

#[derive(Debug, Default, Clone, Copy, Reflect)]
pub struct Tile {
    visibility: TileVisibility,
    index: u32,
}

impl Distribution<Tile> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tile {
        let visibility = match rng.gen_range(0..2) {
            0 => TileVisibility::Empty,
            1 => TileVisibility::Solid,
            _ => unreachable!(),
        };
        Tile {
            visibility,
            index: rng.gen_range(0..4),
        }
    }
}

pub struct ChunkRenderPlugin;

impl Plugin for ChunkRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkCache>()
            .add_systems(Update, update_chunk_meshes);
    }
}

pub fn update_chunk_meshes(
    mut commands: Commands,
    chunks: Query<(Entity, &ChunkData, &ChunkCord, Option<&ChunkLayers>), Changed<ChunkData>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    atlas: Res<super::tile_atlas::TileAtlas>,
) {
    let material: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color_texture: Some(atlas.get_handle()),
        base_color: Color::WHITE,
        alpha_mode: AlphaMode::Opaque,
        ..default()
    });

    for (c, chunk, cord, old_layers) in chunks.iter() {
        if let Some(layer) = old_layers {
            for entity in layer.layers {
                commands.entity(entity).despawn_recursive();
            }
        };

        let mut meshes = vec![];

        meshes.push(meshing::generate_mesh(
            &chunk.tiles[0],
            &chunk.tiles[1],
            &EmptyLayer,
            &atlas,
        ));

        for layer in chunk.tiles.windows(3) {
            meshes.push(meshing::generate_mesh(
                &layer[1], &layer[2], &layer[0], &atlas,
            ))
        }

        meshes.push(meshing::generate_mesh(
            &chunk.tiles[CHUNK_SIZE - 1],
            &EmptyLayer,
            &chunk.tiles[CHUNK_SIZE - 2],
            &atlas,
        ));

        let mut layers = ChunkLayers {
            layers: [Entity::PLACEHOLDER; CHUNK_SIZE],
        };

        let mut current: usize = cord.y as usize * CHUNK_SIZE;

        for (layer, (floor_wall, ceiling)) in layers.layers.iter_mut().zip(meshes) {
            let entity = commands
                .spawn(Layer)
                .insert(SpatialBundle::from_transform(Transform::from_xyz(
                    cord.x as f32,
                    (cord.y as usize + current) as f32,
                    cord.z as f32,
                )))
                .id();
            *layer = entity;

            commands
                .spawn(PbrBundle {
                    mesh: mesh_assets.add(floor_wall),
                    material: material.clone(),
                    ..Default::default()
                })
                .insert(dwarf_map_flags::WallFloorMesh(current))
                .set_parent(entity);

            commands
                .spawn(PbrBundle {
                    mesh: mesh_assets.add(ceiling),
                    material: material.clone(),
                    ..Default::default()
                })
                .insert(dwarf_map_flags::CeilingMesh(current))
                .set_parent(entity);

            current += 1;
        }

        commands.entity(c).insert(layers);
    }
}

#[derive(Resource, Default)]
struct ChunkCache {
    map: HashMap<UVec3, Entity>,
}

impl ChunkCache {
    fn get(&self, pos: &UVec3) -> Option<&Entity> {
        self.map.get(pos)
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct ChunkData {
    tiles: [[[Tile; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

#[allow(unused)]
impl ChunkData {
    pub fn random() -> Self {
        Self {
            tiles: rand::random(),
        }
    }

    pub fn get_tile_local(&self, pos: UVec3) -> &Tile {
        &self.tiles[pos.x as usize][pos.y as usize][pos.z as usize]
    }

    pub fn get_tile_local_mut(&mut self, pos: UVec3) -> &mut Tile {
        &mut self.tiles[pos.x as usize][pos.y as usize][pos.z as usize]
    }

    pub fn set_tile_local(&mut self, pos: UVec3, value: Tile) {
        self.tiles[pos.x as usize][pos.y as usize][pos.z as usize] = value;
    }
}

/// Keeps tack of the layer entities that belong to a chunk
#[derive(Component, Deref)]
pub struct ChunkLayers {
    layers: [Entity; CHUNK_SIZE],
}

/// The Coordinates of a chunk
#[derive(Component, Deref, Default)]
pub struct ChunkCord(UVec3);

#[derive(Bundle, Default)]
pub struct ChunkBundle {
    pub chunk: ChunkData,
    pub cord: ChunkCord,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub global_transform: GlobalTransform,
    pub transform: Transform,
}

#[derive(Debug, Component)]
pub struct Layer;

#[allow(unused)]
impl ChunkBundle {
    pub fn new(chunk: ChunkData, transform: Transform) -> Self {
        Self {
            transform,
            chunk,
            ..Default::default()
        }
    }
}

#[allow(unused)]
#[derive(SystemParam)]
pub struct MapCommands<'w, 's> {
    commands: Commands<'w, 's>,
    cache: Res<'w, ChunkCache>,
    chunks: Query<'w, 's, &'static ChunkData>,
}

#[allow(unused)]
impl<'w, 's> MapCommands<'w, 's> {
    pub fn commands(&mut self) -> Commands {
        self.commands.reborrow()
    }

    /// gets a tile from the world, return panics if the tile is out of bounds.
    pub fn get_tile<'a>(&'a mut self, pos: UVec3) -> TileCommands<'w, 's, 'a> {
        let chunk_pos = pos / UVec3::splat(CHUNK_SIZE as u32);
        let chunk = *self
            .cache
            .get(&chunk_pos)
            .expect("Out of bound tile access");
        TileCommands {
            tile: pos,
            local_tile: pos % UVec3::splat(CHUNK_SIZE as u32),
            chunk,
            map_commands: self,
        }
    }
}

#[allow(unused)]
pub struct TileCommands<'w, 's, 'a> {
    tile: UVec3,
    local_tile: UVec3,
    chunk: Entity,
    map_commands: &'a mut MapCommands<'w, 's>,
}

impl<'w, 's, 'a> TileCommands<'w, 's, 'a> {}
