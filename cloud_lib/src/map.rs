use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rand::prelude::*;
use rand::prelude::Rng;
use std::f32::consts::PI;

use crate::collision::Collider;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum ExitDirection {
    North,
    South,
    East,
    West,
}

#[derive(Component)]
pub struct Wall;

pub struct MapPlugin;

// TODO: very blunt way of tracking map levels. Suitable for game jams and other emergency
// situations.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum LevelState {
    #[default]
    One,
    Two,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<LevelState>()
            .add_plugins(EntropyPlugin::<ChaCha8Rng>::with_seed([1; 32]))
            .add_systems(Startup, prng_setup)
            .add_systems(OnEnter(LevelState::One), generate_level_map);
    }
}

#[derive(Component)]
pub struct Source;

fn prng_setup(mut commands: Commands, mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>) {
    commands.spawn((rng.fork_rng(), Source));
}

fn generate_room(
    mut commands: Commands,
    mut a_rng: Mut<'_, EntropyComponent<ChaCha8Rng>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    height: f32,
    width: f32,
    origin: Vec3,
    exits: Vec<ExitDirection>,
) {
    let mut v = origin.clone();
    let x_side = (width / 18.) as i32;
    let y_side = (height / 18.) as i32;

    let north_exit = exits.contains(&ExitDirection::North);
    for pos in 0..x_side {
        if north_exit && pos > x_side / 3 && pos < x_side * 2 / 3 {
            v.x += 18.;
            continue;
        }
        let color = Color::rgb(a_rng.gen_range(0.0..1.0), a_rng.gen_range(0.0..1.0), 0.);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::RegularPolygon::new(10., 5).into()).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(v)
                    .with_rotation(Quat::from_rotation_z(a_rng.gen_range(0.0..PI))),
                ..default()
            },
            Collider::new(18.),
            Wall,
        ));
        v.x += 18.;
    }

    let east_exit = exits.contains(&ExitDirection::East);
    for pos in 0..y_side {
        if east_exit && pos > y_side / 3 && pos < y_side * 2 / 3 {
            v.y -= 18.;
            continue;
        }
        let color = Color::rgb(a_rng.gen_range(0.0..1.0), a_rng.gen_range(0.0..1.0), 0.);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::RegularPolygon::new(10., 5).into()).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(v)
                    .with_rotation(Quat::from_rotation_z(a_rng.gen_range(0.0..PI))),
                ..default()
            },
            Collider::new(18.),
            Wall,
        ));
        v.y -= 18.;
    }

    let south_exit = exits.contains(&ExitDirection::South);
    for pos in 0..x_side {
        if south_exit && pos > x_side / 3 && pos < x_side * 2 / 3 {
            v.x -= 18.;
            continue;
        }
        let color = Color::rgb(a_rng.gen_range(0.0..1.0), a_rng.gen_range(0.0..1.0), 0.);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::RegularPolygon::new(10., 5).into()).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(v)
                    .with_rotation(Quat::from_rotation_z(a_rng.gen_range(0.0..PI))),
                ..default()
            },
            Collider::new(18.),
            Wall,
        ));
        v.x -= 18.;
    }

    let west_exit = exits.contains(&ExitDirection::West);
    for pos in 0..y_side {
        if west_exit && pos > y_side / 3 && pos < y_side * 2 / 3 {
            v.y += 18.;
            continue;
        }
        let color = Color::rgb(a_rng.gen_range(0.0..1.0), a_rng.gen_range(0.0..1.0), 0.);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::RegularPolygon::new(10., 5).into()).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(v)
                    .with_rotation(Quat::from_rotation_z(a_rng.gen_range(0.0..PI))),
                ..default()
            },
            Collider::new(18.),
            Wall,
        ));
        v.y += 18.;
    }
}

fn generate_level_map(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<&mut EntropyComponent<ChaCha8Rng>, With<Source>>,
) {
    let Ok(a_rng) = query.get_single_mut() else {
        return;
    };

    let height = 400.;
    let width = 800.;
    let origin = Vec3::new(-400., 200., 0.);
    let exits = vec![
        ExitDirection::North,
        ExitDirection::South,
        ExitDirection::East,
        ExitDirection::West,
    ];

    generate_room(
        commands, a_rng, meshes, materials, height, width, origin, exits,
    );
}
