use bevy::{prelude::*, render::texture::ImageSampler};

use crate::prelude::{GameState, LoadingState};

#[derive(Reflect, Resource, Default)]
pub struct LoadingTracker {
    tile_handles: Vec<Handle<Image>>,
}

pub struct DwarfAssetPlugin;

impl Plugin for DwarfAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingTracker>()
            .add_systems(Startup, start_loading)
            .add_systems(
                Update,
                (
                    check_assets_ready.run_if(in_state(LoadingState::LoadingAssets)),
                    build_texture_atlas.run_if(in_state(LoadingState::BuildingAtlas)),
                ),
            );
    }
}

fn start_loading(asset_sever: Res<AssetServer>, mut tracker: ResMut<LoadingTracker>) {
    tracker.tile_handles.push(asset_sever.load("blue.png"));
    tracker.tile_handles.push(asset_sever.load("brown.png"));
    tracker.tile_handles.push(asset_sever.load("green.png"));
    tracker.tile_handles.push(asset_sever.load("orange.png"));
}

fn check_assets_ready(
    server: Res<AssetServer>,
    tracker: Res<LoadingTracker>,
    mut load_state: ResMut<NextState<LoadingState>>,
) {
    use bevy::asset::LoadState;

    let mut done = true;

    for asset in &tracker.tile_handles {
        match server.load_state(asset.id()) {
            LoadState::Loading => {}
            LoadState::Failed => done = false,
            _ => {}
        };
    }

    if done {
        load_state.set(LoadingState::BuildingAtlas)
    }
}

fn build_texture_atlas(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    mut loading: ResMut<NextState<LoadingState>>,
    tracker: Res<LoadingTracker>,
    mut textures: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut builder = TextureAtlasBuilder::default().padding(UVec2::splat(5));
    for img in &tracker.tile_handles {
        let id = img.id();
        builder.add_texture(Some(id), textures.get(id).unwrap())
    }

    let (layout, mut text) = builder.finish().unwrap();
    text.sampler = ImageSampler::nearest();
    let hnd = textures.add(text);
    let tiles = crate::dwarf_map::tile_atlas::TileAtlas {
        image: hnd.clone(),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(hnd.clone()),
            ..default()
        }),
        layout,
    };

    commands.insert_resource(tiles);
    game_state.set(GameState::Playing);
    loading.set(LoadingState::Done);
}
