use bevy::audio::{PlaybackMode, PlaybackSettings, Volume};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rand::prelude::*;
use rand::prelude::Rng;
use std::f32::consts::PI;

use crate::{
    collision::Collider,
    enemy::{CombatStats, Enemy},
    fog::{Fog, FogMaterial, HexlingFogTracker},
    map::{Source, Wall},
    movement::{MovingEntityBundle, Velocity},
    player::{events::SpawnHexlingEvent, HexlingState, Player},
    sound::SoundSettings,
};

const HEXLING_DETERIORATION_FACTOR: f32 = 0.1;
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
            )
            .add_systems(
                Update,
                (maintain_target_list, attack_target).run_if(in_state(HexlingState::Charging)),
            )
            .add_systems(OnExit(crate::GameState::Over), despawn_hexlings);
    }
}

fn hexling_spawner(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut fog_materials: ResMut<Assets<FogMaterial>>,
    mut fog_tracker: ResMut<HexlingFogTracker>,
    mut handle: Query<&Handle<FogMaterial>, With<Fog>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ev_spawn_hexling: EventReader<SpawnHexlingEvent>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut query: Query<&mut EntropyComponent<ChaCha8Rng>, With<Source>>,
    sound_settings: Res<SoundSettings>,
) {
    let Ok(mut a_rng) = query.get_single_mut() else {
        return;
    };
    let Ok(player_transform) = player_query.get_single_mut() else {
        return;
    };
    let Ok(fog_handle) = handle.get_single_mut() else {
        return;
    };
    let fog_material = fog_materials.get_mut(fog_handle).unwrap();

    if !ev_spawn_hexling.is_empty() {
        ev_spawn_hexling.clear();

        commands.spawn((AudioBundle {
            source: asset_server.load("audio/e2.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                volume: Volume::new_relative(sound_settings.effects_volume),
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
        let shape = MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::RegularPolygon::new(HEXLING_RADIUS, 6).into())
                .into(),
            material: materials.add(ColorMaterial::from(color)),
            transform: Transform::from_translation(translation)
                .with_rotation(Quat::from_rotation_z(a_rng.gen_range(0.0..PI))),
            ..default()
        };
        let entity = commands
            .spawn((
                MovingEntityBundle {
                    collider: Collider::new(HEXLING_RADIUS),
                    shape,
                    velocity: Velocity::new(Vec3::ZERO),
                },
                Wall,
                CombatStats {
                    aggro_radius: 50.,
                    attack_range: 10.,
                    attack_rate: 1.,
                    base_damage: 1.,
                    cooldown: 0.,
                    debris_despawn_timer: 0.,
                    health: 10.,
                    target_list: Vec::new(),
                },
            ))
            .insert(Hexling)
            .id();

        // Poke a hole in the fog of war at the hexling's new position
        // TODO: this hideous nightmare is only a gamejam mechanism to offset the fact I can't pass
        // arrays into WGSL. I imagine there's a way to do it with storage buffers, but I hear it's
        // not supported on Web yet?
        // Thank goodness for neovim macros.
        if fog_material.hexling_a == Vec3::ZERO {
            fog_material.hexling_a = translation;
            fog_tracker
                .hexling_entity_positions
                .insert(entity, (translation, "a".to_string()));
        } else if fog_material.hexling_b == Vec3::ZERO {
            fog_material.hexling_b = translation;
            fog_tracker
                .hexling_entity_positions
                .insert(entity, (translation, "b".to_string()));
        } else if fog_material.hexling_c == Vec3::ZERO {
            fog_material.hexling_c = translation;
            fog_tracker
                .hexling_entity_positions
                .insert(entity, (translation, "c".to_string()));
        } else if fog_material.hexling_d == Vec3::ZERO {
            fog_material.hexling_d = translation;
            fog_tracker
                .hexling_entity_positions
                .insert(entity, (translation, "d".to_string()));
        } else if fog_material.hexling_e == Vec3::ZERO {
            fog_material.hexling_e = translation;
            fog_tracker
                .hexling_entity_positions
                .insert(entity, (translation, "e".to_string()));
        } else if fog_material.hexling_f == Vec3::ZERO {
            fog_material.hexling_f = translation;
            fog_tracker
                .hexling_entity_positions
                .insert(entity, (translation, "f".to_string()));
        } else if fog_material.hexling_g == Vec3::ZERO {
            fog_material.hexling_g = translation;
            fog_tracker
                .hexling_entity_positions
                .insert(entity, (translation, "g".to_string()));
        } else if fog_material.hexling_h == Vec3::ZERO {
            fog_material.hexling_h = translation;
            fog_tracker
                .hexling_entity_positions
                .insert(entity, (translation, "h".to_string()));
        } else if fog_material.hexling_i == Vec3::ZERO {
            fog_material.hexling_i = translation;
            fog_tracker
                .hexling_entity_positions
                .insert(entity, (translation, "i".to_string()));
        } else if fog_material.hexling_j == Vec3::ZERO {
            fog_material.hexling_j = translation;
            fog_tracker
                .hexling_entity_positions
                .insert(entity, (translation, "j".to_string()));
        } else if fog_material.hexling_k == Vec3::ZERO {
            fog_material.hexling_k = translation;
            fog_tracker
                .hexling_entity_positions
                .insert(entity, (translation, "k".to_string()));
        } else if fog_material.hexling_l == Vec3::ZERO {
            fog_material.hexling_l = translation;
            fog_tracker
                .hexling_entity_positions
                .insert(entity, (translation, "l".to_string()));
        }
    }
}

fn hexling_recall(
    mut hexling_query: Query<(&mut CombatStats, &Transform, &mut Velocity), With<Hexling>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    for (mut stats, transform, mut velocity) in hexling_query.iter_mut() {
        // Recalling hexlings don't attack anything (for now). Be a good power-up tho.
        stats.target_list.clear();

        let direction = player_transform.translation - transform.translation;
        if direction.length() > MAX_PLAYER_DISTANCE {
            velocity.value = direction.normalize() * HEXLING_SPEED;
        } else {
            velocity.value = Vec3::ZERO;
        }
    }
}

fn hexling_charge(
    enemy_query: Query<&Transform, With<Enemy>>,
    mut hexling_query: Query<(&CombatStats, &Transform, &mut Velocity), With<Hexling>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    for (stats, transform, mut velocity) in hexling_query.iter_mut() {
        let direction = player_transform.translation - transform.translation;
        if stats.target_list.is_empty() {
            velocity.value = -(direction.normalize() * HEXLING_SPEED);
        } else {
            let Ok(target_transform) =
                enemy_query.get(stats.target_list.first().unwrap().to_owned())
            else {
                return;
            };
            let direction = target_transform.translation - transform.translation;
            velocity.value = direction.normalize() * HEXLING_SPEED;
        }
    }
}

// In theory, this could be a generic system. For now, it's convenient to treat it separately for
// hexlings as they have some rather particular behaviour (charge/recall). We also don't have to
// care about the player in the target list.
fn maintain_target_list(
    enemy_query: Query<(Entity, &Transform), (With<Enemy>, Without<Hexling>)>,
    mut query: Query<(&mut CombatStats, &Transform), With<Hexling>>,
) {
    for (mut stats, transform) in query.iter_mut() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let direction = transform.translation - enemy_transform.translation;

            if direction.length() < stats.aggro_radius && !stats.target_list.contains(&enemy_entity)
            {
                stats.target_list.push(enemy_entity);
            }
        }
    }
}

fn attack_target(
    mut enemy_query: Query<(&mut CombatStats, &Transform), (With<Enemy>, Without<Hexling>)>,
    mut query: Query<(&mut CombatStats, &Transform), With<Hexling>>,
    time: Res<Time>,
) {
    for (mut stats, transform) in query.iter_mut() {
        if stats.target_list.is_empty() {
            continue;
        }
        let Ok((mut target_stats, target_transform)) =
            enemy_query.get_mut(stats.target_list.first().unwrap().to_owned())
        else {
            return;
        };
        let distance = (transform.translation - target_transform.translation).length();
        if stats.cooldown <= 0. && target_stats.health > 0. && distance < stats.attack_range {
            target_stats.health -= stats.base_damage;
            stats.cooldown = stats.attack_rate * time.delta_seconds();
            stats.health -= HEXLING_DETERIORATION_FACTOR;
        } else {
            stats.cooldown -= time.delta_seconds();
        }
    }
}

fn despawn_hexlings(mut commands: Commands, query: Query<Entity, With<crate::hexling::Hexling>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
