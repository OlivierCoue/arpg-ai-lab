use bevy::prelude::*;
mod camera;
mod health;
mod player;
mod scenes;
mod ui;
mod world;

use scenes::AppState;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        // Menu (spawn/cleanup camera along with UI)
        .add_systems(OnEnter(AppState::MainMenu), ui::spawn_menu_camera)
        .add_systems(OnEnter(AppState::MainMenu), scenes::menu::setup_menu)
        .add_systems(
            Update,
            scenes::menu::menu_system.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(OnExit(AppState::MainMenu), scenes::menu::cleanup_menu)
        .add_systems(OnExit(AppState::MainMenu), ui::cleanup_menu_camera)
        // InGame
        .add_systems(OnEnter(AppState::InGame), scenes::in_game::setup_game)
        .add_systems(
            Update,
            scenes::in_game::input_system.run_if(in_state(AppState::InGame)),
        )
        // existing plugins
        .add_plugins(health::HealthPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(world::WorldPlugin);

    app.run();
}
