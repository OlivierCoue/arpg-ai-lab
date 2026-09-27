use bevy::prelude::*;

/// Health component storing current and maximum HP.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }

    pub fn apply_damage(&mut self, amount: i32) {
        if amount <= 0 {
            return;
        }
        self.current = (self.current - amount).max(0);
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0
    }
}

/// Marker for entities that reached zero health.
#[derive(Component, Debug)]
pub struct Dead;

/// Damage request sent via Bevy messages.
#[derive(Message, Debug, Clone)]
pub struct DamageRequest {
    pub target: Entity,
    pub amount: i32,
}

/// Resource holding the player's configured maximum health.
#[derive(Resource)]
pub struct PlayerHealthConfig(pub i32);

/// Marker component for the on-screen health text so the UI system can find it.
#[derive(Component)]
pub struct HealthText;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerHealthConfig(100))
            // register the message type so MessageWriter/Reader work
            .add_message::<DamageRequest>()
            .add_systems(Startup, spawn_health_ui)
            // ensure writers run before readers by chaining
            .add_systems(Update, (debug_damage_input_system, damage_system).chain())
            .add_systems(Update, health_ui_system);
    }
}

fn damage_system(
    mut commands: Commands,
    mut deal_damage_reader: MessageReader<DamageRequest>,
    mut query: Query<&mut Health>,
) {
    for msg in deal_damage_reader.read() {
        if let Ok(mut health) = query.get_mut(msg.target) {
            let prev = health.current;
            health.apply_damage(msg.amount);
            if health.current <= 0 && prev > 0 {
                commands.entity(msg.target).insert(Dead);
            }
        }
    }
}

/// Debug input system: writes a DamageRequest message when the debug key is pressed.
fn debug_damage_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut deal_damage_writer: MessageWriter<DamageRequest>,
    player_query: Query<Entity, With<crate::player::Player>>,
) {
    if keyboard.just_pressed(KeyCode::KeyK) {
        if let Ok(player_ent) = player_query.single() {
            deal_damage_writer.write(DamageRequest {
                target: player_ent,
                amount: 10,
            });
        }
    }
}

fn spawn_health_ui(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // UI camera is created elsewhere (camera plugin spawns Camera2d). Spawn a simple top-left Text
    // Parent text node showing label; dynamic value will be a child TextSpan we update.
    let parent = commands
        .spawn((
            Text::new("HP: "),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(5.0),
                top: Val::Px(5.0),
                ..default()
            },
        ))
        .id();

    // Add child span that will carry the dynamic health value and mark it for updates
    commands.entity(parent).with_children(|parent| {
        parent.spawn((TextSpan::new("? / ?"), HealthText));
    });
}

fn health_ui_system(
    query_health: Query<&Health, With<crate::player::Player>>,
    mut query_span: Query<&mut TextSpan, With<HealthText>>,
) {
    let health = match query_health.single() {
        Ok(h) => h,
        Err(_) => return,
    };

    for mut span in query_span.iter_mut() {
        if health.is_dead() {
            **span = "DEAD".to_string();
        } else {
            **span = format!("{} / {}", health.current, health.max);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Resource)]
    struct TestSendDamage {
        target: Entity,
        amount: i32,
    }

    fn send_damage_system(mut writer: MessageWriter<DamageRequest>, test: Res<TestSendDamage>) {
        writer.write(DamageRequest {
            target: test.target,
            amount: test.amount,
        });
    }

    #[test]
    fn starts_at_max_unit() {
        let h = Health::new(50);
        assert_eq!(h.current, 50);
        assert_eq!(h.max, 50);
    }

    #[test]
    fn damage_pipeline_reduces_health_and_marks_dead() {
        let mut app = App::new();
        // register the message type and add systems: sender then damage processor
        app.add_message::<DamageRequest>();
        app.add_systems(Update, (send_damage_system, damage_system).chain());

        // spawn two entities: player and other
        let player = app.world_mut().spawn((Health::new(30),)).id();
        let other = app.world_mut().spawn((Health::new(30),)).id();

        // insert a test resource that will cause send_damage_system to write a message
        app.insert_resource(TestSendDamage {
            target: player,
            amount: 10,
        });

        // run update to process send -> damage_system
        app.update();

        // verify player health reduced and other unchanged
        let player_health = app.world().get::<Health>(player).unwrap();
        let other_health = app.world().get::<Health>(other).unwrap();
        assert_eq!(player_health.current, 20);
        assert_eq!(other_health.current, 30);

        // apply lethal damage by changing the test resource and running another frame
        *app.world_mut().resource_mut::<TestSendDamage>() = TestSendDamage {
            target: player,
            amount: 25,
        };
        app.update();
        let player_health = app.world().get::<Health>(player).unwrap();
        assert_eq!(player_health.current, 0);
        // check Dead component inserted
        assert!(app.world().get::<Dead>(player).is_some());
    }
}
