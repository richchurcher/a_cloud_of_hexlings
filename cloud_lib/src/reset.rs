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
    mut commands: Commands,
    mut next_state: ResMut<NextState<LevelState>>,
    query: Query<Entity, With<crate::enemy::Debris>>,
) {
    println!(":: new_game ::");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    next_state.set(LevelState::One);
}
