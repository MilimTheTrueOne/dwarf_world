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
    y: Res<CurrentMapLayer>,
    mut walls: Query<(&WallFloorMesh, &mut Visibility)>,
) {
    for (wall, mut vis) in walls.iter_mut() {
        *vis = match wall.0 <= y.0 {
            true => Visibility::Visible,
            false => Visibility::Hidden,
        };
    }
}

fn set_ceiling_visibility(
    y: Res<CurrentMapLayer>,
    mut ceilings: Query<(&CeilingMesh, &mut Visibility)>,
) {
    for (ceiling, mut vis) in ceilings.iter_mut() {
        *vis = match ceiling.0 == y.0 {
            true => Visibility::Visible,
            false => Visibility::Hidden,
        };
    }
}
