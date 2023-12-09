use bevy::audio::{AudioBundle, PlaybackMode, PlaybackSettings};
use bevy::prelude::*;

use crate::{GameState, LevelState};

pub struct ResetPlugin;

impl Plugin for ResetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, new_game)
            .add_systems(OnExit(GameState::Over), new_game);
    }
}

fn new_game(
    asset_server: ResMut<AssetServer>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<LevelState>>,
    query: Query<Entity, With<crate::enemy::Debris>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    next_state.set(LevelState::One);

    commands.spawn((AudioBundle {
        source: asset_server.load("audio/six_sides.ogg"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Once,
            ..default()
        },
    },));
}
