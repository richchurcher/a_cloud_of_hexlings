use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use std::f32::consts::PI;

use crate::collision::Collider;
use crate::player::events::{ChargeEvent, RecallEvent};
use crate::player::Player;
use crate::GameState;

const CHARGE_COLOR: Color = Color::rgb(3.25, 2.4, 1.1);
const RECALL_COLOR: Color = Color::rgb(0.25, 0.4, 0.1);

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
pub struct MovingEntityBundle {
    pub collider: Collider,
    pub shape: MaterialMesh2dBundle<ColorMaterial>,
    pub velocity: Velocity,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_position.run_if(in_state(GameState::Playing)),
                flip_player.run_if(in_state(GameState::Playing)),
            ),
        );
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}

fn flip_player(
    mut animations: ResMut<Assets<AnimationClip>>,
    mut ev_charge: EventReader<ChargeEvent>,
    mut ev_recall: EventReader<RecallEvent>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&mut AnimationPlayer, Entity, &Handle<ColorMaterial>, &Name), With<Player>>,
) {
    let Ok((mut animation_player, _entity, material_handle, name)) = query.get_single_mut() else {
        return;
    };

    for _ in ev_charge.read() {
        let mut animation = AnimationClip::default();

        animation.add_curve_to_path(
            EntityPath {
                parts: vec![name.clone()],
            },
            VariableCurve {
                keyframe_timestamps: vec![0.0, 0.25, 0.5],
                keyframes: Keyframes::Rotation(vec![
                    Quat::IDENTITY,
                    Quat::from_axis_angle(Vec3::Y, PI / 2.),
                    Quat::IDENTITY,
                ]),
            },
        );
        let animation_handle = animations.add(animation);
        animation_player.play(animation_handle);
        if let Some(material) = materials.get_mut(material_handle) {
            material.color = CHARGE_COLOR;
        }
    }

    for _ in ev_recall.read() {
        let mut animation = AnimationClip::default();

        animation.add_curve_to_path(
            EntityPath {
                parts: vec![name.clone()],
            },
            VariableCurve {
                keyframe_timestamps: vec![0.0, 0.25, 0.5],
                keyframes: Keyframes::Rotation(vec![
                    Quat::IDENTITY,
                    Quat::from_axis_angle(Vec3::Y, PI / 2.),
                    Quat::IDENTITY,
                ]),
            },
        );
        let animation_handle = animations.add(animation);
        animation_player.play(animation_handle);
        if let Some(material) = materials.get_mut(material_handle) {
            material.color = RECALL_COLOR;
        }
    }
}

