use bevy::prelude::*;

pub struct AssetsPlugin;

#[derive(Component)]
pub struct SpawnHexlingEffect;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, preload);
    }
}

fn preload(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("assets/audio/e2.wav"),
            ..default()
        },
        SpawnHexlingEffect,
    ));
}
