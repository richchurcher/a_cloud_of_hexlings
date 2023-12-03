use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<cloud_lib::GameState>()
        .add_systems(Startup, setup)
        .add_plugins(cloud_lib::menu::MenuPlugin)
        .add_plugins(cloud_lib::pause_menu::PauseMenuPlugin)
        .add_plugins(cloud_lib::player::PlayerPlugin)
        .add_plugins(cloud_lib::debug::DebugPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
