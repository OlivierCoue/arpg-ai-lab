use bevy::prelude::*;

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
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement_system);
    }
}

fn spawn_player(mut commands: Commands) {
    // Spawn a simple visible sprite as the player
    commands.spawn((
        Sprite::from_color(Color::srgb(0.3, 0.7, 0.9), Vec2::new(32.0, 32.0)),
        Player,
    ));
}

fn player_movement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    speed: Res<PlayerSpeed>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let dt = time.delta_secs();
    let vel = compute_velocity_from_input(&keyboard, speed.0, dt);
    if vel != Vec2::ZERO {
        for mut transform in query.iter_mut() {
            transform.translation.x += vel.x;
            transform.translation.y += vel.y;
        }
    }
}

/// Compute velocity vector from keyboard input; kept small so it can be tested.
pub fn compute_velocity_from_input(input: &ButtonInput<KeyCode>, speed: f32, dt: f32) -> Vec2 {
    let up = input.pressed(KeyCode::KeyW);
    let down = input.pressed(KeyCode::KeyS);
    let left = input.pressed(KeyCode::KeyA);
    let right = input.pressed(KeyCode::KeyD);
    compute_velocity_from_bools(up, down, left, right, speed, dt)
}

/// Testable pure function: compute movement delta from booleans
pub fn compute_velocity_from_bools(
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    speed: f32,
    dt: f32,
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
    dir = dir.normalize() * speed * dt;
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
        let v = compute_velocity_from_bools(false, false, false, false, 100.0, 0.016);
        assert_eq!(v, Vec2::ZERO);
    }

    #[test]
    fn test_up_movement() {
        let speed = 100.0;
        let dt = 0.5;
        let v = compute_velocity_from_bools(true, false, false, false, speed, dt);
        assert!(approx_eq(v, Vec2::new(0.0, speed * dt)));
    }

    #[test]
    fn test_left_movement() {
        let speed = 120.0;
        let dt = 0.25;
        let v = compute_velocity_from_bools(false, false, true, false, speed, dt);
        assert!(approx_eq(v, Vec2::new(-speed * dt, 0.0)));
    }

    #[test]
    fn test_diagonal_normalization() {
        let speed = 50.0;
        let dt = 0.1;
        let v = compute_velocity_from_bools(true, false, false, true, speed, dt); // up + right
        let expected_component = (speed * dt) / 2f32.sqrt();
        assert!(approx_eq(
            v,
            Vec2::new(expected_component, expected_component)
        ));
        let len = v.length();
        assert!((len - speed * dt).abs() < EPS);
    }
}
