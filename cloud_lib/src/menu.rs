use bevy::prelude::*;

use crate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), init)
            .add_systems(Update, menu.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), despawn_thing::<MainMenuScreen>);
    }
}

#[derive(Component)]
struct MainMenuScreen;

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
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            },
            MainMenuScreen,
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "a cloud of hexlings",
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

fn menu(
    mut exit: EventWriter<bevy::app::AppExit>,
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        next_state.set(GameState::Playing);
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(bevy::app::AppExit);
    }
}

pub fn despawn_thing<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
