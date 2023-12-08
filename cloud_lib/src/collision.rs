use bevy::{prelude::*, render::primitives::Aabb, sprite::collide_aabb, utils::HashMap};

use crate::hexling::{Hexling, HEXLING_SPEED};
use crate::map::Wall;
use crate::movement::Velocity;
use crate::player::{Player, SPEED};

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<(Entity, collide_aabb::Collision)>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                collision_detection,
                handle_player_collisions,
                handle_hexling_collisions,
            )
                .chain(),
        );
    }
}

fn collision_detection(mut query: Query<(Entity, &Aabb, &Transform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<(Entity, collide_aabb::Collision)>> =
        HashMap::new();

    for (entity_a, aabb_a, transform_a, _) in query.iter() {
        for (entity_b, aabb_b, transform_b, _) in query.iter() {
            if entity_a != entity_b {
                // TODO: Why does using the `.center` of the Aabb here result in very odd numbers
                // for the player's x value? e.g. -3.x * 10^-6 or some such nonsense.
                match collide_aabb::collide(
                    transform_a.translation,
                    Vec2::new(aabb_a.half_extents.x * 2., aabb_a.half_extents.y * 2.),
                    transform_b.translation,
                    Vec2::new(aabb_b.half_extents.x * 2., aabb_b.half_extents.y * 2.),
                ) {
                    Some(c) => {
                        colliding_entities
                            .entry(entity_a)
                            .or_insert_with(Vec::new)
                            .push((entity_b, c));
                    }
                    None => {}
                }
            }
        }
    }

    for (entity, _, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}

// TODO: remove this monstrosity at the earliest opportunity, and replace it with a proper
// collision system.
fn handle_player_collisions(
    mut query: Query<(&Collider, &mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
    wall_query: Query<&Wall>,
) {
    for (collider, mut transform, mut velocity) in query.iter_mut() {
        for &collided_entity in collider.colliding_entities.iter() {
            if wall_query.get(collided_entity.0).is_ok() {
                match collided_entity.1 {
                    collide_aabb::Collision::Top => {
                        // transform.translation.y += (SPEED / 1.25) * time.delta_seconds();
                        velocity.value.y += SPEED / 2.;
                    }
                    collide_aabb::Collision::Bottom => {
                        // transform.translation.y -= (SPEED / 1.25) * time.delta_seconds();
                        velocity.value.y -= SPEED / 2.;
                    }
                    collide_aabb::Collision::Left => {
                        // transform.translation.x -= (SPEED / 1.25) * time.delta_seconds();
                        velocity.value.x -= SPEED / 2.;
                    }
                    collide_aabb::Collision::Right => {
                        // transform.translation.x += (SPEED / 1.25) * time.delta_seconds();
                        velocity.value.x += SPEED / 2.;
                    }
                    collide_aabb::Collision::Inside => {
                        // transform.translation.y += (SPEED / 1.25) * time.delta_seconds();
                        // transform.translation.x += (SPEED / 1.25) * time.delta_seconds();
                        velocity.value.y += SPEED / 2.;
                        velocity.value.x += SPEED / 2.;
                    }
                }
            }
        }
    }
}

// TODO: duplication, expedient for now
fn handle_hexling_collisions(
    mut query: Query<(&Collider, &mut Transform), (With<Hexling>, Without<Player>)>,
    time: Res<Time>,
) {
    for (collider, mut transform) in query.iter_mut() {
        for &collided_entity in collider.colliding_entities.iter() {
            match collided_entity.1 {
                collide_aabb::Collision::Top => {
                    transform.translation.y += HEXLING_SPEED * time.delta_seconds();
                }
                collide_aabb::Collision::Bottom => {
                    transform.translation.y -= HEXLING_SPEED * time.delta_seconds();
                }
                collide_aabb::Collision::Left => {
                    transform.translation.x -= HEXLING_SPEED * time.delta_seconds();
                }
                collide_aabb::Collision::Right => {
                    transform.translation.x += HEXLING_SPEED * time.delta_seconds();
                }
                collide_aabb::Collision::Inside => {
                    transform.translation.x += HEXLING_SPEED * time.delta_seconds();
                    transform.translation.y += HEXLING_SPEED * time.delta_seconds();
                }
            }
        }
    }
}
