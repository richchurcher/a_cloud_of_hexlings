use bevy::prelude::*;

use crate::player::events::SpawnHexlingEvent;

pub struct HexlingPlugin;

impl Plugin for HexlingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hexling_spawner);
    }
}

fn hexling_spawner(mut ev_spawn_hexling: EventReader<SpawnHexlingEvent>) {
    for _ in ev_spawn_hexling.read() {
        println!(" ::: HEXLING SPAWN REQUESTED ::: ");
    }
}
