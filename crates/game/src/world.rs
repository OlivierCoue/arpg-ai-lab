use crate::scenes::AppState;
use bevy::prelude::*;

/// Size of each checkerboard tile in world units
#[derive(Resource)]
pub struct TileSize(pub f32);

/// Half-extent of the checkerboard in tiles (total width = 2*extent + 1)
#[derive(Resource)]
pub struct GridExtent(pub i32);

/// Marker component for world tiles so they can be cleaned up when leaving InGame
#[derive(Component)]
pub struct WorldTile;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileSize(32.0))
            .insert_resource(GridExtent(20)) // 41x41 tiles by default
            .add_systems(OnEnter(AppState::InGame), spawn_checkerboard)
            .add_systems(OnExit(AppState::InGame), cleanup_checkerboard);
    }
}

fn spawn_checkerboard(mut commands: Commands, tile_size: Res<TileSize>, extent: Res<GridExtent>) {
    let ts = tile_size.0;
    let ext = extent.0;

    // Create a simple black-and-white checkerboard aligned to world coordinates.
    // Tiles are spawned with Z = -1.0 so they render behind the player (player at z=0).
    for x in -ext..=ext {
        for y in -ext..=ext {
            let is_white = ((x + y) & 1) == 0;
            let color = if is_white { Color::WHITE } else { Color::BLACK };
            commands.spawn((
                Sprite::from_color(color, Vec2::splat(ts)),
                Transform::from_xyz(x as f32 * ts, y as f32 * ts, -1.0),
                WorldTile,
            ));
        }
    }
}

fn cleanup_checkerboard(mut commands: Commands, query: Query<Entity, With<WorldTile>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}
