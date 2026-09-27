use crate::player::Player;
use crate::scenes::AppState;
use bevy::prelude::*;
use bevy::transform::TransformSystems;

/// Configurable camera offset resource (world units)
#[derive(Resource)]
pub struct CameraOffset(pub Vec3);

/// Marker for the gameplay camera so exactly one is present
#[derive(Component)]
pub struct GameCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraOffset(Vec3::ZERO))
            .add_systems(OnEnter(AppState::InGame), spawn_camera)
            .add_systems(
                PostUpdate,
                camera_follow_system
                    .before(TransformSystems::Propagate)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::InGame), cleanup_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    // Spawn exactly one 2D camera for gameplay and tag it so we can find it later
    // Use the simple Camera2d marker so exactly one 2D camera is present
    commands.spawn((Camera2d, GameCamera));
}

fn cleanup_camera(mut commands: Commands, query: Query<Entity, With<GameCamera>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

pub(crate) fn camera_follow_system(
    player_query: Query<&Transform, (With<Player>, Without<GameCamera>)>,
    mut cam_query: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
    offset: Res<CameraOffset>,
) {
    let player_tf = match player_query.single() {
        Ok(t) => t,
        Err(_) => return, // no player yet
    };

    for mut cam_tf in cam_query.iter_mut() {
        cam_tf.translation.x = player_tf.translation.x + offset.0.x;
        cam_tf.translation.y = player_tf.translation.y + offset.0.y;
        // keep existing z so camera remains at its default depth
    }
}
