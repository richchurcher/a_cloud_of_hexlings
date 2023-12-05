use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "a cloud of hexlings".to_string(),
                // Bind to canvas included in `index.html`
                canvas: Some("#bevy".to_owned()),
                // The canvas size is constrained in index.html and build/web/styles.css
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<cloud_lib::GameState>()
        .add_systems(Startup, setup)
        .add_plugins(cloud_lib::menu::MenuPlugin)
        .add_plugins(cloud_lib::pause_menu::PauseMenuPlugin)
        .add_plugins(cloud_lib::player::PlayerPlugin)
        .add_plugins(cloud_lib::collision::CollisionPlugin)
        .add_plugins(cloud_lib::movement::MovementPlugin)
        .add_plugins(cloud_lib::map::MapPlugin)
        .add_plugins(cloud_lib::hexling::HexlingPlugin)
        .add_plugins(cloud_lib::debug::DebugPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
    ));
}
