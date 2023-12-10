use bevy::{asset::AssetMetaCheck, prelude::*};

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
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
        .add_state::<cloud_lib::LevelState>()
        .add_plugins(cloud_lib::camera::CameraPlugin)
        .add_plugins(cloud_lib::menu::MenuPlugin)
        .add_plugins(cloud_lib::pause_menu::PauseMenuPlugin)
        .add_plugins(cloud_lib::over_menu::OverMenuPlugin)
        .add_plugins(cloud_lib::sound::SoundPlugin)
        .add_plugins(cloud_lib::reset::ResetPlugin)
        .add_plugins(cloud_lib::player::PlayerPlugin)
        .add_plugins(cloud_lib::fog::FogPlugin)
        .add_plugins(cloud_lib::collision::CollisionPlugin)
        .add_plugins(cloud_lib::movement::MovementPlugin)
        .add_plugins(cloud_lib::map::MapPlugin)
        .add_plugins(cloud_lib::hexling::HexlingPlugin)
        .add_plugins(cloud_lib::enemy::EnemyPlugin)
        .run();
}
