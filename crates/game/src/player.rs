use bevy::prelude::*;

use crate::health::Health;
use crate::scenes::AppState;

/// Marker component for the player entity
#[derive(Component)]
pub struct Player;

/// Simple resource to control player speed (pixels per second)
#[derive(Resource)]
pub struct PlayerSpeed(pub f32);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerSpeed(200.0))
            .add_systems(OnEnter(AppState::InGame), spawn_player)
            .add_systems(
                Update,
                player_movement_system.run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::InGame), cleanup_player);
    }
}

fn spawn_player(mut commands: Commands, config: Res<crate::health::PlayerHealthConfig>) {
    // Spawn a simple visible sprite as the player with an explicit Transform so it appears in world space
    commands.spawn((
        Sprite::from_color(Color::srgb(0.3, 0.7, 0.9), Vec2::new(32.0, 32.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
        Health::new(config.0),
    ));
}

fn cleanup_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

fn player_movement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    speed: Res<PlayerSpeed>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let dt = time.delta_secs();
    let vel = compute_velocity_from_input(&keyboard, speed.0);
    if vel != Vec2::ZERO {
        let delta = vel * dt; // convert velocity (units/sec) to per-frame delta
        for mut transform in query.iter_mut() {
            transform.translation.x += delta.x;
            transform.translation.y += delta.y;
        }
    }
}

/// Compute velocity vector (units per second) from keyboard input; kept small so it can be tested.
pub fn compute_velocity_from_input(input: &ButtonInput<KeyCode>, speed: f32) -> Vec2 {
    let up = input.pressed(KeyCode::KeyW);
    let down = input.pressed(KeyCode::KeyS);
    let left = input.pressed(KeyCode::KeyA);
    let right = input.pressed(KeyCode::KeyD);
    compute_velocity_from_bools(up, down, left, right, speed)
}

/// Testable pure function: compute velocity vector (units per second) from booleans
pub fn compute_velocity_from_bools(
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    speed: f32,
) -> Vec2 {
    let mut dir = Vec2::ZERO;
    if up {
        dir.y += 1.0;
    }
    if down {
        dir.y -= 1.0;
    }
    if left {
        dir.x -= 1.0;
    }
    if right {
        dir.x += 1.0;
    }
    if dir == Vec2::ZERO {
        return Vec2::ZERO;
    }
    dir = dir.normalize() * speed; // velocity (units/sec)
    dir
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f32 = 1e-4;

    fn approx_eq(a: Vec2, b: Vec2) -> bool {
        (a.x - b.x).abs() < EPS && (a.y - b.y).abs() < EPS
    }

    #[test]
    fn test_no_input() {
        let v = compute_velocity_from_bools(false, false, false, false, 100.0);
        assert_eq!(v, Vec2::ZERO);
    }

    #[test]
    fn test_up_movement() {
        let speed = 100.0;
        let v = compute_velocity_from_bools(true, false, false, false, speed);
        assert!(approx_eq(v, Vec2::new(0.0, speed)));
    }

    #[test]
    fn test_left_movement() {
        let speed = 120.0;
        let v = compute_velocity_from_bools(false, false, true, false, speed);
        assert!(approx_eq(v, Vec2::new(-speed, 0.0)));
    }

    #[test]
    fn test_diagonal_normalization() {
        let speed = 50.0;
        let v = compute_velocity_from_bools(true, false, false, true, speed); // up + right
        let expected_component = speed / 2f32.sqrt();
        assert!(approx_eq(
            v,
            Vec2::new(expected_component, expected_component)
        ));
        let len = v.length();
        assert!((len - speed).abs() < EPS);
    }

    #[test]
    fn test_delta_time_scaling_equivalence() {
        // Verify that applying velocity * total_time equals repeated steps summing to the same total time.
        let speed = 80.0;
        let v = compute_velocity_from_bools(true, false, false, false, speed); // up
        let total_time = 0.123_f32;

        // single-step displacement
        let single = v * total_time;

        // multi-step displacement (e.g., 7 steps)
        let steps = 7u32;
        let dt = total_time / steps as f32;
        let mut multi = Vec2::ZERO;
        for _ in 0..steps {
            multi += v * dt;
        }

        assert!(approx_eq(single, multi));
    }

    #[test]
    fn test_compute_velocity_from_input_mapping() {
        // Ensure compute_velocity_from_input maps KeyCode presses to the same result as compute_velocity_from_bools
        let mut keyboard = ButtonInput::<KeyCode>::default();
        keyboard.press(KeyCode::KeyW);
        let speed = 90.0;
        let v_input = compute_velocity_from_input(&keyboard, speed);
        let v_bools = compute_velocity_from_bools(true, false, false, false, speed);
        assert!(approx_eq(v_input, v_bools));

        // also verify left mapping
        let mut keyboard2 = ButtonInput::<KeyCode>::default();
        keyboard2.press(KeyCode::KeyA);
        let v_input2 = compute_velocity_from_input(&keyboard2, speed);
        let v_bools2 = compute_velocity_from_bools(false, false, true, false, speed);
        assert!(approx_eq(v_input2, v_bools2));
    }

    #[test]
    fn test_player_movement_system_integration_headless() {
        use bevy::time::TimePlugin;
        use std::time::Duration;

        let mut app = App::new();
        // headless: only add minimal non-rendering plugins
        app.add_plugins(TimePlugin);

        // add system under test
        app.add_systems(Update, player_movement_system);

        // resources
        app.insert_resource(PlayerSpeed(100.0));

        // simulate keyboard with W pressed (we insert the ButtonInput ourselves to avoid InputPlugin overwriting it)
        let mut keyboard = ButtonInput::<KeyCode>::default();
        keyboard.press(KeyCode::KeyW);
        app.insert_resource(keyboard);

        // ensure time exists via TimePlugin and advance by dt
        {
            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(Duration::from_secs_f32(0.2));
        }

        // spawn player entity at origin
        app.world_mut()
            .spawn((Transform::from_xyz(0.0, 0.0, 0.0), Player));

        // inspect transform before update
        {
            let mut query_state = app.world_mut().query::<(&Transform, &Player)>();
            for (transform, _player) in query_state.iter(app.world()) {
                println!(
                    "before update y={} expected={}",
                    transform.translation.y,
                    100.0 * 0.2
                );
            }
        }

        // run the system directly using SystemState to avoid schedule interactions
        use bevy::ecs::system::SystemState;

        {
            let world = app.world_mut();
            #[allow(clippy::type_complexity)]
            let mut system_state: SystemState<(
                Res<ButtonInput<KeyCode>>,
                Res<Time>,
                Res<PlayerSpeed>,
                Query<&mut Transform, With<Player>>,
            )> = SystemState::new(world);

            let (keyboard_res, time_res, speed_res, query) = system_state
                .get_mut(world)
                .expect("failed to get system params");

            // call the system function directly with the fetched params
            player_movement_system(keyboard_res, time_res, speed_res, query);
        }

        // inspect transform after update and verify translation.y == speed * dt
        let mut found = false;
        let mut query_state = app.world_mut().query::<(&Transform, &Player)>();
        for (transform, _player) in query_state.iter(app.world()) {
            println!(
                "after update y={} expected={}",
                transform.translation.y,
                100.0 * 0.2
            );
            assert!((transform.translation.y - (100.0 * 0.2)).abs() < EPS);
            found = true;
        }
        assert!(found, "Player entity not found in world query");
    }

    #[test]
    fn test_camera_follow_headless() {
        use bevy::time::TimePlugin;
        use std::time::Duration;

        let mut app = App::new();
        // headless: only add minimal non-rendering plugins
        app.add_plugins(TimePlugin);

        // add systems: player movement (Update) and camera follow (PostUpdate)
        app.add_systems(Update, player_movement_system);
        app.add_systems(PostUpdate, crate::camera::camera_follow_system);

        // resources
        app.insert_resource(PlayerSpeed(100.0));

        // non-zero camera offset to verify it's applied
        app.insert_resource(crate::camera::CameraOffset(Vec3::new(2.0, -1.0, 0.0)));

        // simulate keyboard with W pressed
        let mut keyboard = ButtonInput::<KeyCode>::default();
        keyboard.press(KeyCode::KeyW);
        app.insert_resource(keyboard);

        // advance time by dt
        {
            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(Duration::from_secs_f32(0.2));
        }

        // spawn player entity at origin
        app.world_mut()
            .spawn((Transform::from_xyz(0.0, 0.0, 0.0), Player));

        // spawn camera entity with an initial z so we can verify z is preserved
        let initial_cam_z = 10.0_f32;
        app.world_mut().spawn((
            Transform::from_xyz(0.0, 0.0, initial_cam_z),
            crate::camera::GameCamera,
        ));

        // run the full app update so Update then PostUpdate systems run in order
        app.update();

        // query results
        let mut player_tf_q = app.world_mut().query::<(&Transform, &Player)>();
        let mut cam_tf_q = app
            .world_mut()
            .query::<(&Transform, &crate::camera::GameCamera)>();

        let player_pos = player_tf_q
            .iter(app.world())
            .next()
            .expect("player missing")
            .0
            .translation;
        let cam_pos = cam_tf_q
            .iter(app.world())
            .next()
            .expect("camera missing")
            .0
            .translation;

        // expected camera position = player position + offset
        let offset = app.world().resource::<crate::camera::CameraOffset>().0;
        assert!((cam_pos.x - (player_pos.x + offset.x)).abs() < EPS);
        assert!((cam_pos.y - (player_pos.y + offset.y)).abs() < EPS);
        // z should be preserved
        assert!((cam_pos.z - initial_cam_z).abs() < EPS);
    }
}
