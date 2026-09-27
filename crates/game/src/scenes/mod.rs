use bevy::prelude::*;

pub mod in_game;
pub mod menu;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}
