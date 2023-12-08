pub mod events;

use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::collision::Collider;
use crate::enemy::CombatStats;
use crate::movement::{MovingEntityBundle, Velocity};
use crate::GameState;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum HexlingState {
    #[default]
    Recalling,
    Charging,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum HexlingSpawnState {
    #[default]
    Idle,
    Spawning,
}

pub const CHARGE_COLOR: Color = Color::rgb(3.25, 2.4, 1.1);
const PLAYER_RADIUS: f32 = 30.;
pub const RECALL_COLOR: Color = Color::rgb(0.25, 0.4, 0.1);
const SPAWN_KEY_MS: u128 = 1500;
pub const SPEED: f32 = 200.;
const STARTING_HEALTH: f32 = 50.;
pub const STARTING_TRANSLATION: Vec3 = Vec3::new(200., 0., 0.);

#[derive(Default, Resource)]
pub struct SpawnKeyHeld {
    pub duration: u128,
}

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player.run_if(run_once()))
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
            .add_systems(
                Update,
                (
                    hexling_spawn.run_if(in_state(HexlingSpawnState::Idle)),
                    hexling_spawn_reset.run_if(in_state(HexlingSpawnState::Spawning)),
                ),
            )
            .add_event::<events::ChargeEvent>()
            .add_event::<events::RecallEvent>()
            .add_event::<events::SpawnHexlingEvent>()
            .add_state::<HexlingSpawnState>()
            .add_state::<HexlingState>();
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::RegularPolygon::new(PLAYER_RADIUS, 6).into())
            .into(),
        material: materials.add(ColorMaterial::from(RECALL_COLOR)),
        transform: Transform::from_translation(STARTING_TRANSLATION),
        ..default()
    };

    commands
        .spawn((
            AnimationPlayer::default(),
            CombatStats {
                aggro_radius: 0.,
                attack_range: 0.,
                attack_rate: 0.,
                base_damage: 0.,
                cooldown: 0.,
                health: STARTING_HEALTH,
                target_list: Vec::new(),
            },
            MovingEntityBundle {
                collider: Collider::new(PLAYER_RADIUS),
                shape,
                velocity: Velocity::new(Vec3::ZERO),
            },
            Name::new("player"),
        ))
        .insert(Player);
}

fn player_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let Ok(mut velocity) = query.get_single_mut() else {
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
}

fn hexling_spawn(
    mut ev_spawn_hexling: EventWriter<events::SpawnHexlingEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<HexlingSpawnState>>,
    mut query: Query<Entity, With<Player>>,
    mut spawn_key_held: ResMut<SpawnKeyHeld>,
    time: Res<Time>,
) {
    let Ok(entity) = query.get_single_mut() else {
        return;
    };
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        spawn_key_held.duration += time.delta().as_millis();
        if spawn_key_held.duration >= SPAWN_KEY_MS {
            ev_spawn_hexling.send(events::SpawnHexlingEvent(entity));
            next_state.set(HexlingSpawnState::Spawning);
        }
    }
}

fn hexling_spawn_reset(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<HexlingSpawnState>>,
    mut spawn_key_held: ResMut<SpawnKeyHeld>,
) {
    if keyboard_input.just_released(KeyCode::ShiftLeft) {
        spawn_key_held.duration = 0;
        next_state.set(HexlingSpawnState::Idle);
    }
}

fn hexling_recall(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut ev_recall: EventWriter<events::RecallEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<HexlingState>>,
    mut query: Query<Entity, With<Player>>,
) {
    let Ok(entity) = query.get_single_mut() else {
        return;
    };
    if keyboard_input.just_released(KeyCode::Space) {
        ev_recall.send(events::RecallEvent(entity));
        next_state.set(HexlingState::Recalling);
        commands.spawn((AudioBundle {
            source: asset_server.load("audio/a.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                ..default()
            },
        },));
    }
}

fn hexling_charge(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut ev_charge: EventWriter<events::ChargeEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<HexlingState>>,
    mut query: Query<Entity, With<Player>>,
) {
    let Ok(entity) = query.get_single_mut() else {
        return;
    };
    if keyboard_input.just_released(KeyCode::Space) {
        // Is this smart? Probably not, but it makes a neat effect, and is slightly different with
        // its timings each time!
        ev_charge.send(events::ChargeEvent(entity));
        next_state.set(HexlingState::Charging);
        commands.spawn((AudioBundle {
            source: asset_server.load("audio/e.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                ..default()
            },
        },));
        commands.spawn((AudioBundle {
            source: asset_server.load("audio/e2.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                ..default()
            },
        },));
        commands.spawn((AudioBundle {
            source: asset_server.load("audio/a.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                ..default()
            },
        },));
        commands.spawn((AudioBundle {
            source: asset_server.load("audio/b.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                ..default()
            },
        },));
        commands.spawn((AudioBundle {
            source: asset_server.load("audio/g.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                ..default()
            },
        },));
        commands.spawn((AudioBundle {
            source: asset_server.load("audio/fsharp3.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                ..default()
            },
        },));
    }
}
