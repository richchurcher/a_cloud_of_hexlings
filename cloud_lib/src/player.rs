pub mod events;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::collision::Collider;
use crate::movement::{MovingEntityBundle, Velocity};
use crate::GameState;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum HexlingState {
    #[default]
    Recalling,
    Charging,
}

const CHARGE_COLOR: Color = Color::rgb(3.25, 2.4, 1.1);
const RECALL_COLOR: Color = Color::rgb(0.25, 0.4, 0.1);
const STARTING_TRANSLATION: Vec3 = Vec3::new(200., 0., 0.);
const SPEED: f32 = 200.;

#[derive(Default, Resource)]
pub struct SpawnKeyHeld {
    pub duration: u128,
}

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .init_resource::<SpawnKeyHeld>()
            .add_systems(Update, player_controls.run_if(in_state(GameState::Playing)))
            .add_systems(
                Update,
                hexling_recall.run_if(in_state(HexlingState::Charging)),
            )
            .add_systems(
                Update,
                hexling_charge.run_if(in_state(HexlingState::Recalling)),
            )
            .add_event::<events::ChargeEvent>()
            .add_event::<events::RecallEvent>()
            .add_event::<events::SpawnHexlingEvent>()
            .add_state::<HexlingState>();
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(RECALL_COLOR)),
        transform: Transform::from_translation(STARTING_TRANSLATION),
        ..default()
    };

    commands
        .spawn((
            MovingEntityBundle {
                collider: Collider::new(50.),
                // rigid_body: RigidBody::Dynamic,
                shape,
                velocity: Velocity::new(Vec3::ZERO),
            },
            AnimationPlayer::default(),
            Name::new("player"),
        ))
        .insert(Player);
}

fn player_controls(
    mut ev_spawn_hexling: EventWriter<events::SpawnHexlingEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Velocity), With<Player>>,
    mut spawn_key_held: ResMut<SpawnKeyHeld>,
    time: Res<Time>,
) {
    let Ok((entity, mut velocity)) = query.get_single_mut() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::W) {
        velocity.value.y = SPEED;
    } else if keyboard_input.pressed(KeyCode::S) {
        velocity.value.y = -SPEED;
    } else {
        velocity.value.y = 0.;
    }

    if keyboard_input.pressed(KeyCode::A) {
        velocity.value.x = -SPEED;
    } else if keyboard_input.pressed(KeyCode::D) {
        velocity.value.x = SPEED;
    } else {
        velocity.value.x = 0.;
    }

    if keyboard_input.pressed(KeyCode::Space) {
        spawn_key_held.duration += time.delta().as_millis();
    }

    if keyboard_input.just_released(KeyCode::Space) {
        if spawn_key_held.duration >= 2000 {
            println!("spawn_key_held.duration: {}", spawn_key_held.duration);
            ev_spawn_hexling.send(events::SpawnHexlingEvent(entity));
        }
        spawn_key_held.duration = 0;
    }
}

fn hexling_recall(
    mut ev_recall: EventWriter<events::RecallEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<HexlingState>>,
    mut query: Query<Entity, With<Player>>,
    mut spawn_key_held: ResMut<SpawnKeyHeld>,
) {
    let Ok(entity) = query.get_single_mut() else {
        return;
    };
    if keyboard_input.just_released(KeyCode::Space) {
        if spawn_key_held.duration < 2000 {
            println!("spawn_key_held.duration: {}", spawn_key_held.duration);
            ev_recall.send(events::RecallEvent(entity));
            next_state.set(HexlingState::Recalling);
            spawn_key_held.duration = 0;
        }
    }
}

fn hexling_charge(
    mut ev_charge: EventWriter<events::ChargeEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<HexlingState>>,
    mut query: Query<Entity, With<Player>>,
    mut spawn_key_held: ResMut<SpawnKeyHeld>,
) {
    let Ok(entity) = query.get_single_mut() else {
        return;
    };
    if keyboard_input.just_released(KeyCode::Space) {
        if spawn_key_held.duration < 2000 {
            println!("spawn_key_held.duration: {}", spawn_key_held.duration);
            ev_charge.send(events::ChargeEvent(entity));
            next_state.set(HexlingState::Charging);
            spawn_key_held.duration = 0;
        }
    }
}
