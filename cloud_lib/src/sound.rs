use bevy::{
    audio::{PlaybackMode, Volume, VolumeLevel},
    prelude::*,
};

use crate::GameState;

const MAX_VOLUME_DB: f32 = 20.;
const MIN_VOLUME_DB: f32 = -80.;
const VOLUME_STEP_DB: f32 = 5.;

#[derive(Resource)]
pub struct SoundSettings {
    pub effects_on: bool,
    // Expressed as a fraction of global volume.
    pub effects_volume: f32,
    pub global_sound_on: bool,
    pub global_volume_db: f32,
    pub soundtrack_on: bool,
    // Expressed as a fraction of global volume.
    pub soundtrack_volume: f32,
}

#[derive(Component)]
pub struct Soundtrack;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), soundtrack.run_if(run_once()))
            .add_systems(OnExit(GameState::Over), soundtrack)
            .add_systems(
                OnEnter(GameState::Over),
                crate::menu::despawn_thing::<Soundtrack>,
            )
            .add_systems(OnEnter(GameState::Paused), toggle_soundtrack)
            .add_systems(OnExit(GameState::Paused), toggle_soundtrack)
            .add_systems(Update, sound_controls.run_if(in_state(GameState::Playing)))
            .insert_resource(SoundSettings {
                effects_on: true,
                effects_volume: 0.5,
                global_sound_on: true,
                global_volume_db: 1.0,
                soundtrack_on: true,
                soundtrack_volume: 1.0,
            });
    }
}

fn soundtrack(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    sound_settings: Res<SoundSettings>,
) {
    commands
        .spawn((AudioBundle {
            source: asset_server.load("audio/six_sides.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new_relative(sound_settings.soundtrack_volume),
                ..default()
            },
        },))
        .insert(Soundtrack);
}

fn sound_controls(
    mut effects_query: Query<&mut AudioSink, Without<Soundtrack>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut sound_settings: ResMut<SoundSettings>,
    mut global_volume: ResMut<GlobalVolume>,
    soundtrack_query: Query<&AudioSink, With<Soundtrack>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if let Ok(soundtrack) = soundtrack_query.get_single() {
            soundtrack.toggle();
        };
    }

    if keyboard_input.just_pressed(KeyCode::Plus) && sound_settings.global_volume_db < MAX_VOLUME_DB
    {
        // Need to do two things: first, change the global level for all future sounds.
        sound_settings.global_volume_db = sound_settings.global_volume_db + VOLUME_STEP_DB;
        let amplitude = global_volume.volume.get() * 10_f32.powf(VOLUME_STEP_DB / 20.0);
        global_volume.volume = VolumeLevel::new(amplitude);

        // Second, alter all currently playing sinks.
        for sink in soundtrack_query.iter() {
            sink.set_volume(amplitude * sound_settings.soundtrack_volume)
        }
        for sink in effects_query.iter_mut() {
            sink.set_volume(amplitude * sound_settings.effects_volume)
        }
    }

    if keyboard_input.just_pressed(KeyCode::Minus)
        && sound_settings.global_volume_db > MIN_VOLUME_DB
    {
        sound_settings.global_volume_db = sound_settings.global_volume_db - VOLUME_STEP_DB;
        let amplitude = global_volume.volume.get() * 10_f32.powf(-VOLUME_STEP_DB / 20.0);
        global_volume.volume = VolumeLevel::new(amplitude);

        // Second, alter all currently playing sinks.
        for sink in soundtrack_query.iter() {
            sink.set_volume(amplitude * sound_settings.soundtrack_volume)
        }
        for sink in effects_query.iter_mut() {
            sink.set_volume(amplitude * sound_settings.effects_volume)
        }
    }
}

fn toggle_soundtrack(query: Query<&AudioSink, With<Soundtrack>>) {
    if let Ok(soundtrack) = query.get_single() {
        soundtrack.toggle();
    };
}
