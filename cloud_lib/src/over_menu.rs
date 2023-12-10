use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;

use crate::sound::SoundSettings;
use crate::GameState;

pub struct OverMenuPlugin;

impl Plugin for OverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Over), init)
            .add_systems(Update, menu.run_if(in_state(GameState::Over)))
            .add_systems(OnExit(GameState::Over), despawn_screen::<OverMenuScreen>);
    }
}

#[derive(Component)]
struct OverMenuScreen;

fn init(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    sound_settings: Res<SoundSettings>,
) {
    let settings = PlaybackSettings {
        mode: PlaybackMode::Loop,
        volume: Volume::new_relative(sound_settings.effects_volume),
        ..default()
    };
    commands.spawn((AudioBundle {
        source: asset_server.load("audio/thud.ogg"),
        settings,
    },));
    commands.spawn((AudioBundle {
        source: asset_server.load("audio/tap.ogg"),
        settings,
    },));
    commands.spawn((AudioBundle {
        source: asset_server.load("audio/e.ogg"),
        settings,
    },));
    commands.spawn((AudioBundle {
        source: asset_server.load("audio/d.ogg"),
        settings,
    },));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::Rgba {
                    alpha: 0.5,
                    blue: 0.,
                    green: 0.,
                    red: 0.,
                }),
                ..Default::default()
            },
            OverMenuScreen,
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "game over",
                TextStyle {
                    font_size: 36.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            builder.spawn(TextBundle::from_section(
                "enter for new game, esc to quit",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

fn menu(keyboard_input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        next_state.set(GameState::Playing);
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
