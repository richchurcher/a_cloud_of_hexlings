use crate::collision::Collider;
use crate::movement::{MovingEntityBundle, Velocity};
use crate::LevelState;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::GameState;

const COLOR: Color = Color::rgb(0.9, 0.0, 0.1);
const RADIUS: f32 = 20.;
const ORBIT_POINT: Vec3 = Vec3::new(-250., 350., 0.);
const SPEED: f32 = 50.;
const STARTING_TRANSLATION: Vec3 = Vec3::new(-300., 400., 0.);

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::One), spawn_enemy.run_if(run_once()))
            .add_systems(
                Update,
                (maintain_target_list, passive_motion, aggro_motion)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
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
    pub base_damage: u32,
    // When cooldown reduces to 0, an attack can be made. Starts at 0, reduces by
    // time.delta_seconds() each tick.
    pub cooldown: f32,
    pub health: u32,
    // This list contains all targets. They may not still be within aggro_radius. The list may be
    // re-ordered, and the first entity on the list will always be the primary target.
    pub target_list: Vec<Entity>,
}

fn spawn_enemy(
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
                attack_rate: 1.,
                base_damage: 1,
                cooldown: 0.,
                health: 20,
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
) {
    for (mut stats, transform) in enemy_query.iter_mut() {
        for (friendly_entity, friendly_transform) in friendly_query.iter() {
            let direction = transform.translation - friendly_transform.translation;
            if direction.length() < stats.aggro_radius
                && !stats.target_list.contains(&friendly_entity)
            {
                // TODO: this implements "last target acquired is most important", which is
                // probably not what we want.
                stats.target_list.push(friendly_entity);
            }
        }
    }
}
