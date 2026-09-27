use bevy::prelude::*;
mod camera;
mod health;
mod player;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(health::HealthPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(world::WorldPlugin)
        .run();
}
