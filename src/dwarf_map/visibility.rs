use super::{dwarf_map_flags::*, CurrentMapLayer};
use bevy::prelude::*;

/// Plugin to handle the visibility of layer meshes.
/// In the future might handle animations
pub struct LayerVisibilityPlugin;

impl Plugin for LayerVisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (set_ceiling_visibility, set_floor_visibility));
    }
}

fn set_floor_visibility(
    mut commands: Commands,
    y: Res<CurrentMapLayer>,
    walls: Query<(Entity, &WallMesh)>,
) {
    for (entity, wall) in &walls {
        if wall.0 > y.0 {
            commands.entity(entity).insert(Visibility::Hidden);
        } else {
            commands.entity(entity).insert(Visibility::Visible);
        };
    }
}

fn set_ceiling_visibility(
    mut commands: Commands,
    y: Res<CurrentMapLayer>,
    ceilings: Query<(Entity, &CeilingMesh)>,
) {
    for (entity, ceiling) in &ceilings {
        if ceiling.0 == y.0 {
            commands.entity(entity).insert(Visibility::Visible);
        } else {
            commands.entity(entity).insert(Visibility::Hidden);
        };
    }
}
