use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::movement::{MovingEntityBundle, Velocity};
use crate::GameState;

const COLOR: Color = Color::WHITE;
const STARTING_TRANSLATION: Vec3 = Vec3::new(200., 0., 100.);
const SPEED: f32 = 200.;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, player_controls);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(COLOR)),
        transform: Transform::from_translation(STARTING_TRANSLATION),
        ..default()
    };

    commands.spawn((
        MovingEntityBundle {
            shape,
            velocity: Velocity::new(Vec3::ZERO),
        },
        Player,
    ));
}

fn player_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
    // time: Res<Time>,
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
