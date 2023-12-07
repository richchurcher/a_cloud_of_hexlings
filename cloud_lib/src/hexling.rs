use bevy::audio::{PlaybackMode, PlaybackSettings};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rand::prelude::*;
use rand::prelude::Rng;
use std::f32::consts::PI;

use crate::collision::Collider;
use crate::map::{Source, Wall};
use crate::player::{events::SpawnHexlingEvent, HexlingState, Player};

const HEXLING_RADIUS: f32 = 6.;
pub const HEXLING_SPEED: f32 = 200.;
const MIN_PLAYER_DISTANCE: f32 = 65.;
const MAX_PLAYER_DISTANCE: f32 = 85.;

#[derive(Component)]
pub struct Hexling;

pub struct HexlingPlugin;

impl Plugin for HexlingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hexling_spawner)
            .add_systems(
                Update,
                hexling_recall.run_if(in_state(HexlingState::Recalling)),
            )
            .add_systems(
                Update,
                hexling_charge.run_if(in_state(HexlingState::Charging)),
            );
    }
}

fn hexling_spawner(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ev_spawn_hexling: EventReader<SpawnHexlingEvent>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut query: Query<&mut EntropyComponent<ChaCha8Rng>, With<Source>>,
) {
    let Ok(mut a_rng) = query.get_single_mut() else {
        return;
    };
    let Ok(player_transform) = player_query.get_single_mut() else {
        return;
    };
    if !ev_spawn_hexling.is_empty() {
        ev_spawn_hexling.clear();

        commands.spawn((AudioBundle {
            source: asset_server.load("audio/e2.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                ..default()
            },
        },));

        // Various greens
        let color = Color::rgb(
            a_rng.gen_range(0.0..0.1),
            a_rng.gen_range(0.7..1.0),
            a_rng.gen_range(0.0..0.1),
        );
        let translation = Vec3::new(
            player_transform.translation.x
                + a_rng.gen_range(MIN_PLAYER_DISTANCE..MAX_PLAYER_DISTANCE),
            player_transform.translation.y
                + a_rng.gen_range(MIN_PLAYER_DISTANCE..MAX_PLAYER_DISTANCE),
            0.,
        );
        commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::RegularPolygon::new(HEXLING_RADIUS, 6).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(color)),
                    transform: Transform::from_translation(translation)
                        .with_rotation(Quat::from_rotation_z(a_rng.gen_range(0.0..PI))),
                    ..default()
                },
                Collider::new(HEXLING_RADIUS),
                Wall,
            ))
            .insert(Hexling);
    }
}

fn hexling_recall(
    mut hexling_query: Query<&mut Transform, (With<Hexling>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    for mut hexling_transform in hexling_query.iter_mut() {
        let direction = player_transform.translation - hexling_transform.translation;
        if direction.length() > MAX_PLAYER_DISTANCE {
            hexling_transform.translation +=
                direction.normalize() * HEXLING_SPEED * time.delta_seconds();
        }
        if direction.length() < MIN_PLAYER_DISTANCE {
            // Slowly adjust back to normal orbit distance
            hexling_transform.translation -= direction.normalize() * 2.;
        }
    }
}

fn hexling_charge(
    mut hexling_query: Query<&mut Transform, (With<Hexling>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    for mut hexling_transform in hexling_query.iter_mut() {
        let direction = player_transform.translation - hexling_transform.translation;
        hexling_transform.translation -=
            direction.normalize() * HEXLING_SPEED * time.delta_seconds();
    }
}
