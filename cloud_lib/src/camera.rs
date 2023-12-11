use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

use crate::{
    movement::Velocity,
    player::{Player, STARTING_TRANSLATION},
    GameState,
};

const CAMERA_PLAYER_DISTANCE: f32 = 200.;
const CAMERA_SPEED: f32 = 200.;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, camera_follow.run_if(in_state(GameState::Playing)));
    }
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
        BloomSettings::default(),
        Velocity::new(Vec3::ZERO),
    ));
}

fn camera_follow(
    mut camera_query: Query<(&Transform, &mut Velocity), With<Camera>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let Ok((camera_transform, mut velocity)) = camera_query.get_single_mut() else {
        return;
    };
    let direction = player_transform.translation - camera_transform.translation;
    if direction.length() > CAMERA_PLAYER_DISTANCE {
        // Allows a little "slop" before camera will move, but as player approaches edge of screen,
        // camera should begin to follow it, eventually almost-but-not-quite catching up.
        velocity.value = direction.normalize() * CAMERA_SPEED;
    }

    if direction.length() < 200. {
        velocity.value = Vec3::ZERO;
    }
}
