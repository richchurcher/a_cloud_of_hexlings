use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::GameState;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    shape: MaterialMesh2dBundle<ColorMaterial>,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::RegularPolygon::new(100., 6).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::rgb(1.25, 1.4, 1.1))),
        transform: Transform::from_translation(Vec3::new(200., 0., 100.)),
        ..default()
    };

    commands.spawn(PlayerBundle { shape }).insert(Player);
}
