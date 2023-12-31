use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use crate::collision::Collider;
use crate::movement::{MovingEntityBundle, Velocity};
use crate::player::Player;
use crate::sound::SoundSettings;
use crate::GameState;

pub const COLOR: Color = Color::rgb(0.9, 0.0, 0.1);
pub const RADIUS: f32 = 20.;
const ORBIT_POINT: Vec3 = Vec3::new(-250., 350., 0.);
const SPEED: f32 = 50.;
pub const STARTING_TRANSLATION: Vec3 = Vec3::new(-300., 400., 0.);

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Debris {
    pub despawn_timer: f32,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy)
            .add_systems(OnExit(GameState::Over), spawn_enemy)
            .add_systems(OnEnter(GameState::Over), despawn_enemy)
            .add_systems(
                Update,
                (
                    maintain_target_list,
                    passive_motion,
                    aggro_motion,
                    splodey,
                    despawn_debris,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, attack_target.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct CombatStats {
    // Any entity within this radius will be added to the target_list.
    pub aggro_radius: f32,
    // Maximum range expressed as distance to target's centre from self's centre.
    pub attack_range: f32,
    // Amount by which cooldown is increased following each attack.
    pub attack_rate: f32,
    pub base_damage: f32,
    // When cooldown reduces to 0, an attack can be made. Starts at 0, reduces by
    // time.delta_seconds() each tick.
    pub cooldown: f32,
    pub debris_despawn_timer: f32,
    pub health: f32,
    // This list contains all targets. They may not still be within aggro_radius. The list may be
    // re-ordered, and the first entity on the list will always be the primary target.
    pub target_list: Vec<Entity>,
}

pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::RegularPolygon::new(RADIUS, 8).into())
            .into(),
        material: materials.add(ColorMaterial::from(COLOR)),
        transform: Transform::from_translation(STARTING_TRANSLATION),
        ..default()
    };

    commands
        .spawn((
            AnimationPlayer::default(),
            CombatStats {
                aggro_radius: 200.,
                attack_range: 100.,
                attack_rate: 10.,
                base_damage: 1.,
                cooldown: 0.,
                debris_despawn_timer: 10.,
                health: 3.,
                target_list: Vec::new(),
            },
            MovingEntityBundle {
                collider: Collider::new(RADIUS),
                shape,
                velocity: Velocity::new(Vec3::ZERO),
            },
            Name::new("enemy"),
        ))
        .insert(Enemy);

    commands
        .spawn((
            AnimationPlayer::default(),
            CombatStats {
                aggro_radius: 200.,
                attack_range: 100.,
                attack_rate: 10.,
                base_damage: 1.,
                cooldown: 0.,
                debris_despawn_timer: 10.,
                health: 3.,
                target_list: Vec::new(),
            },
            MovingEntityBundle {
                collider: Collider::new(RADIUS),
                shape: MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::RegularPolygon::new(RADIUS, 8).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(COLOR)),
                    transform: Transform::from_translation(Vec3::new(300., 400., 0.)),
                    ..default()
                },
                velocity: Velocity::new(Vec3::ZERO),
            },
            Name::new("enemy"),
        ))
        .insert(Enemy);

    commands
        .spawn((
            AnimationPlayer::default(),
            CombatStats {
                aggro_radius: 200.,
                attack_range: 100.,
                attack_rate: 10.,
                base_damage: 1.,
                cooldown: 0.,
                debris_despawn_timer: 10.,
                health: 3.,
                target_list: Vec::new(),
            },
            MovingEntityBundle {
                collider: Collider::new(RADIUS),
                shape: MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::RegularPolygon::new(RADIUS, 8).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(COLOR)),
                    transform: Transform::from_translation(Vec3::new(-300., -400., 0.)),
                    ..default()
                },
                velocity: Velocity::new(Vec3::ZERO),
            },
            Name::new("enemy"),
        ))
        .insert(Enemy);

    commands
        .spawn((
            AnimationPlayer::default(),
            CombatStats {
                aggro_radius: 200.,
                attack_range: 100.,
                attack_rate: 10.,
                base_damage: 1.,
                cooldown: 0.,
                debris_despawn_timer: 10.,
                health: 3.,
                target_list: Vec::new(),
            },
            MovingEntityBundle {
                collider: Collider::new(RADIUS),
                shape: MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::RegularPolygon::new(RADIUS, 8).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(COLOR)),
                    transform: Transform::from_translation(Vec3::new(300., -400., 0.)),
                    ..default()
                },
                velocity: Velocity::new(Vec3::ZERO),
            },
            Name::new("enemy"),
        ))
        .insert(Enemy);
}

fn passive_motion(
    mut query: Query<(&CombatStats, &mut Transform, &mut Velocity), With<Enemy>>,
    time: Res<Time>,
) {
    for (stats, mut transform, mut velocity) in query.iter_mut() {
        transform.rotate_z(3. * time.delta_seconds());
        if !stats.target_list.is_empty() {
            // Has at least one target: passive motion doesn't apply
            continue;
        }

        // Orbit fixed point
        let direction = (ORBIT_POINT - transform.translation).normalize();
        let perpendicular = Vec3::new(-direction.y, direction.x, 0.);
        velocity.value = perpendicular * SPEED;
    }
}

fn aggro_motion(
    mut enemy_query: Query<(&CombatStats, &mut Transform, &mut Velocity), With<Enemy>>,
    friendly_query: Query<&Transform, (With<CombatStats>, Without<Enemy>)>,
) {
    for (stats, transform, mut velocity) in enemy_query.iter_mut() {
        if stats.target_list.is_empty() {
            // No targets. Leash to spawn point.
            continue;
        }

        let Ok(target) = friendly_query.get(stats.target_list.first().unwrap().to_owned()) else {
            return;
        };

        let direction = target.translation - transform.translation;
        velocity.value = direction.normalize() * SPEED;
    }
}

fn maintain_target_list(
    mut enemy_query: Query<(&mut CombatStats, &Transform), With<Enemy>>,
    friendly_query: Query<(Entity, &Transform), (With<CombatStats>, Without<Enemy>)>,
    player_query: Query<Entity, With<Player>>,
) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    for (mut stats, transform) in enemy_query.iter_mut() {
        for (friendly_entity, friendly_transform) in friendly_query.iter() {
            let direction = transform.translation - friendly_transform.translation;
            if direction.length() < stats.aggro_radius
                && !stats.target_list.contains(&friendly_entity)
            {
                stats.target_list.push(friendly_entity);
            }
        }

        // Reorder target list for priority:
        //   - kill player first. Player must die.
        //   - kill closest hexling only if player is not on the target list
        stats.target_list.sort_by(|a, b| {
            if a == &player_entity {
                return std::cmp::Ordering::Less;
            }
            if b == &player_entity {
                return std::cmp::Ordering::Greater;
            }

            // OK let's go with the most inefficient approach first...
            let Ok((_, a_transform)) = friendly_query.get(*a) else {
                return std::cmp::Ordering::Equal;
            };
            let Ok((_, b_transform)) = friendly_query.get(*b) else {
                return std::cmp::Ordering::Equal;
            };
            let a_distance = (transform.translation - a_transform.translation).length();
            let b_distance = (transform.translation - b_transform.translation).length();
            match a_distance < b_distance {
                true => std::cmp::Ordering::Less,
                false => std::cmp::Ordering::Greater,
            }
        });
    }
}

fn attack_target(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut enemy_query: Query<(&mut CombatStats, &Transform), With<Enemy>>,
    mut friendly_query: Query<(&mut CombatStats, &Transform), Without<Enemy>>,
    sound_settings: Res<SoundSettings>,
    time: Res<Time>,
) {
    for (mut stats, transform) in enemy_query.iter_mut() {
        if stats.target_list.is_empty() {
            continue;
        }
        let Ok((mut target_stats, target_transform)) =
            friendly_query.get_mut(stats.target_list.first().unwrap().to_owned())
        else {
            return;
        };
        let distance = (transform.translation - target_transform.translation).length();
        if stats.cooldown <= 0. && target_stats.health > 0. && distance < stats.attack_range {
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/enemy_basic_attack.ogg"),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Once,
                    volume: Volume::new_relative(sound_settings.effects_volume / 2.),
                    ..default()
                },
            });

            target_stats.health -= stats.base_damage;
            stats.cooldown = stats.attack_rate * time.delta_seconds();
        } else {
            stats.cooldown -= time.delta_seconds();
        }
    }
}

fn splodey(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &CombatStats, &Transform), With<Enemy>>,
    sound_settings: Res<SoundSettings>,
) {
    for (entity, stats, transform) in query.iter() {
        if stats.health <= 0. {
            for _ in 0..20 {
                let shape = MaterialMesh2dBundle {
                    mesh: meshes.add(shape::RegularPolygon::new(6., 3).into()).into(),
                    material: materials.add(ColorMaterial::from(COLOR)),
                    transform: Transform::from_translation(transform.translation).with_rotation(
                        Quat::from_rotation_z(rand::random::<f32>() * 2. * std::f32::consts::PI),
                    ),
                    ..default()
                };

                commands
                    .spawn(MovingEntityBundle {
                        collider: Collider::new(6.),
                        shape,
                        velocity: Velocity::new(Vec3::ZERO),
                    })
                    .insert(Debris { despawn_timer: 10. });
            }

            commands.entity(entity).despawn_recursive();

            let settings = PlaybackSettings {
                mode: PlaybackMode::Once,
                volume: Volume::new_relative(sound_settings.effects_volume),
                ..default()
            };
            commands.spawn((AudioBundle {
                source: asset_server.load("audio/enemy_c.ogg"),
                settings,
            },));
            commands.spawn((AudioBundle {
                source: asset_server.load("audio/enemy_f.ogg"),
                settings,
            },));
            commands.spawn((AudioBundle {
                source: asset_server.load("audio/enemy_g.ogg"),
                settings,
            },));
            commands.spawn((AudioBundle {
                source: asset_server.load("audio/enemy_a.ogg"),
                settings,
            },));
        }
    }
}

fn despawn_debris(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Debris)>,
    time: Res<Time>,
) {
    for (entity, mut debris) in query.iter_mut() {
        debris.despawn_timer -= time.delta_seconds();
        if debris.despawn_timer <= 0. {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_enemy(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
