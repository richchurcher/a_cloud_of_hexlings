use bevy::prelude::*;

use crate::GameState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), init)
            .add_systems(Update, pause_key.run_if(in_state(GameState::Playing)))
            .add_systems(Update, menu.run_if(in_state(GameState::Paused)))
            .add_systems(OnExit(GameState::Paused), despawn_screen::<PauseMenuScreen>);
    }
}

#[derive(Component)]
struct PauseMenuScreen;

fn init(mut commands: Commands) {
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
            PauseMenuScreen,
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "paused",
                TextStyle {
                    font_size: 36.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            builder.spawn(TextBundle::from_section(
                "enter to play, esc to quit",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

fn pause_key(keyboard_input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
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
