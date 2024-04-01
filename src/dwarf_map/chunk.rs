use bevy::{ecs::system::SystemParam, prelude::*, utils::HashMap};
use rand::{distributions::Standard, prelude::*};

pub mod data;
pub mod meshing;
mod temp_mesh;
pub use meshing::*;

use super::dwarf_map_flags;
use crate::prelude::*;

pub const CHUNK_SIZE: usize = 16;

#[derive(Debug, Default, Clone, Copy, Reflect)]
pub struct Tile {
    visibility: TileVisibility,
    index: usize,
}

impl Distribution<Tile> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tile {
        let visibility = match rng.gen_range(0..2) {
            0 => TileVisibility::Solid,
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
        app.init_resource::<ChunkCache>().add_systems(
            Update,
            update_chunk_meshes.run_if(in_state(GameState::Playing)),
        );
    }
}

pub fn update_chunk_meshes(
    mut commands: Commands,
    chunks: Query<(Entity, &ChunkData, &ChunkCord, Option<&ChunkLayers>), Changed<ChunkData>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    atlas: Res<super::tile_atlas::TileAtlas>,
    cache: Res<ChunkCache>,
) {
    let material = atlas.material.clone();
    let dummy = ChunkData::default();

    for (c, chunk, cord, old_layers) in chunks.iter() {
        if let Some(layer) = old_layers {
            for entity in layer.layers {
                commands.entity(entity).despawn_recursive();
            }
        };

        let mut meshes: Vec<(Mesh, Mesh)> = vec![];

        let neigh = cache.get_neighbors(cord.0);

        chunk.gen_meshes(
            neigh[0].map_or(&dummy, |e| chunks.get(e).unwrap().1),
            neigh[1].map_or(&dummy, |e| chunks.get(e).unwrap().1),
            neigh[2].map_or(&dummy, |e| chunks.get(e).unwrap().1),
            neigh[3].map_or(&dummy, |e| chunks.get(e).unwrap().1),
            neigh[4].map_or(&dummy, |e| chunks.get(e).unwrap().1),
            neigh[5].map_or(&dummy, |e| chunks.get(e).unwrap().1),
            &atlas,
            &mut meshes,
        );

        let mut layers = ChunkLayers {
            layers: [Entity::PLACEHOLDER; CHUNK_SIZE],
        };

        let mut current: usize = cord.y as usize * CHUNK_SIZE;

        for (layer, (floor_wall, ceiling)) in layers.layers.iter_mut().zip(meshes) {
            let entity = commands
                .spawn(ChunkLayer)
                .insert(SpatialBundle::from_transform(Transform::from_xyz(
                    0.0,
                    (current - cord.y as usize * CHUNK_SIZE) as f32 - 8.0,
                    0.0,
                )))
                .set_parent(c)
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
pub struct ChunkCache {
    map: HashMap<UVec3, Entity>,
}

impl ChunkCache {
    pub fn get(&self, pos: &UVec3) -> Option<Entity> {
        self.map.get(pos).copied()
    }

    pub fn insert(&mut self, pos: UVec3, e: Entity) {
        self.map.insert(pos, e);
    }

    fn get_neighbors(&self, pos: UVec3) -> [Option<Entity>; 6] {
        [
            self.get(&pos.wrapping_add(UVec3::X)),
            self.get(&pos.wrapping_sub(UVec3::X)),
            self.get(&pos.wrapping_add(UVec3::Z)),
            self.get(&pos.wrapping_sub(UVec3::Z)),
            self.get(&pos.wrapping_add(UVec3::Y)),
            self.get(&pos.wrapping_sub(UVec3::Y)),
        ]
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct ChunkData {
    tiles: [[[Tile; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

const MAX: usize = CHUNK_SIZE - 1;

impl ChunkData {
    #[allow(clippy::too_many_arguments)]
    pub fn gen_meshes(
        &self,
        chunk_right: &ChunkData,
        chunk_left: &ChunkData,
        chunk_front: &ChunkData,
        chunk_back: &ChunkData,
        chunk_top: &ChunkData,
        chunk_bottom: &ChunkData,
        atlas: &Res<super::tile_atlas::TileAtlas>,
        meshes: &mut Vec<(Mesh, Mesh)>,
    ) {
        let get_vis = |pos: UVec2, layer_index: usize| -> [TileVisibility; 6] {
            let x = pos.x as usize;
            let y = pos.y as usize;

            let layer = self.tiles[layer_index];

            let (left, right) = match x {
                0 => (
                    chunk_left.tiles[layer_index][y][MAX].visibility,
                    layer[x + 1][y].visibility,
                ),
                MAX => (
                    layer[x - 1][y].visibility,
                    chunk_right.tiles[layer_index][y][0].visibility,
                ),
                _ => (layer[x - 1][y].visibility, layer[x + 1][y].visibility),
            };

            let (front, back) = match y {
                0 => (
                    chunk_front.tiles[layer_index][x][MAX].visibility,
                    layer[x][y + 1].visibility,
                ),
                MAX => (
                    layer[x][y - 1].visibility,
                    chunk_back.tiles[layer_index][x][0].visibility,
                ),
                _ => (layer[x][y - 1].visibility, layer[x][y + 1].visibility),
            };

            let (above, below) = match layer_index {
                0 => (self.tiles[layer_index + 1], chunk_bottom.tiles[MAX]),
                MAX => (self.tiles[layer_index - 1], chunk_top.tiles[0]),
                _ => (self.tiles[layer_index + 1], self.tiles[layer_index - 1]),
            };

            [
                above[x][y].visibility,
                below[x][y].visibility,
                right,
                left,
                front,
                back,
            ]
        };

        for (i, layer) in self.tiles.iter().enumerate() {
            meshes.push(meshing::generate_mesh(layer, get_vis, i, atlas));
        }
    }
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
#[derive(Component, Deref, Default, Clone, Copy)]
pub struct ChunkCord(pub UVec3);

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
pub struct ChunkLayer;

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
        let chunk = self
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
