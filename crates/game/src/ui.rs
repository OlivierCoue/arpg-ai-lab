use bevy::prelude::*;

/// Marker for the main-menu camera so it can be cleaned up on exit
#[derive(Component)]
pub struct MainMenuCamera;

/// Spawn a camera used only for the Main Menu UI
pub fn spawn_menu_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainMenuCamera));
}

/// Cleanup any main-menu camera entities
pub fn cleanup_menu_camera(mut commands: Commands, query: Query<Entity, With<MainMenuCamera>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}
