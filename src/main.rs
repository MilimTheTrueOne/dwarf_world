use bevy::{prelude::*, window::PresentMode};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use prelude::LoadingState;
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

mod assets;
mod dwarf_map;
mod states;

pub mod prelude {
    pub use super::states::*;
}

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa::Off)
        .insert_state(states::GameState::Loading)
        .insert_state(LoadingState::LoadingAssets)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Dwarf Fortress like 3D".into(),
                name: Some("bevy.app".into()),
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LookTransformPlugin)
        .add_plugins(FpsCameraPlugin::default())
        .add_plugins((dwarf_map::DwarfMapPlugin, assets::DwarfAssetPlugin))
        .add_systems(Startup, setup);
    //#[cfg(debug_assertions)]
    {
        use bevy::diagnostic::LogDiagnosticsPlugin;
        app.add_plugins(LogDiagnosticsPlugin::default());
        use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        app.add_plugins(ResourceInspectorPlugin::<Msaa>::new())
    };
    app.run();
}

fn setup(mut commands: Commands) {
    // camera
    commands
        .spawn(Camera3dBundle::default())
        .insert(FpsCameraBundle::new(
            FpsCameraController::default(),
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
