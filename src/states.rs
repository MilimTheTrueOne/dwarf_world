use bevy::prelude::*;

#[derive(Debug, States, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Loading,
    Playing,
}
