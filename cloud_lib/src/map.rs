use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rand::prelude::*;
use rand::prelude::Rng;
use std::f32::consts::PI;

use crate::collision::Collider;

const BASE_COLOR_LOW_END: f32 = 0.3;
const BASE_COLOR_HIGH_END: f32 = 0.5;
const ROOM_WIDTH: f32 = 400.;
const ROOM_HEIGHT: f32 = 200.;
const WALL_RADIUS: f32 = 9.;
const WARMTH_LOW_END: f32 = 0.4;
const WARMTH_HIGH_END: f32 = 0.6;

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
    let x_side = (width / WALL_RADIUS) as i32;
    let y_side = (height / WALL_RADIUS) as i32;

    let north_exit = exits.contains(&ExitDirection::North);
    for pos in 0..x_side {
        if north_exit && pos > x_side / 3 && pos < x_side * 2 / 3 {
            v.x += WALL_RADIUS * 2.;
            continue;
        }
        let warmth = a_rng.gen_range(WARMTH_LOW_END..WARMTH_HIGH_END);
        let base_color = a_rng.gen_range(BASE_COLOR_LOW_END..BASE_COLOR_HIGH_END);
        let color = Color::rgb(warmth, base_color, base_color);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::RegularPolygon::new(WALL_RADIUS, 4).into())
                    .into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(v)
                    .with_rotation(Quat::from_rotation_z(a_rng.gen_range(0.0..PI))),
                ..default()
            },
            Collider::new(WALL_RADIUS),
            Wall,
        ));
        v.x += WALL_RADIUS * 2.;
    }

    let east_exit = exits.contains(&ExitDirection::East);
    for pos in 0..y_side {
        if east_exit && pos > y_side / 3 && pos < y_side * 2 / 3 {
            v.y -= WALL_RADIUS * 2.;
            continue;
        }
        let warmth = a_rng.gen_range(WARMTH_LOW_END..WARMTH_HIGH_END);
        let base_color = a_rng.gen_range(BASE_COLOR_LOW_END..BASE_COLOR_HIGH_END);
        let color = Color::rgb(warmth, base_color, base_color);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::RegularPolygon::new(WALL_RADIUS, 4).into())
                    .into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(v)
                    .with_rotation(Quat::from_rotation_z(a_rng.gen_range(0.0..PI))),
                ..default()
            },
            Collider::new(WALL_RADIUS),
            Wall,
        ));
        v.y -= WALL_RADIUS * 2.;
    }

    let south_exit = exits.contains(&ExitDirection::South);
    for pos in 0..x_side {
        if south_exit && pos > x_side / 3 && pos < x_side * 2 / 3 {
            v.x -= WALL_RADIUS * 2.;
            continue;
        }
        let warmth = a_rng.gen_range(WARMTH_LOW_END..WARMTH_HIGH_END);
        let base_color = a_rng.gen_range(BASE_COLOR_LOW_END..BASE_COLOR_HIGH_END);
        let color = Color::rgb(warmth, base_color, base_color);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::RegularPolygon::new(WALL_RADIUS, 4).into())
                    .into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(v)
                    .with_rotation(Quat::from_rotation_z(a_rng.gen_range(0.0..PI))),
                ..default()
            },
            Collider::new(WALL_RADIUS),
            Wall,
        ));
        v.x -= WALL_RADIUS * 2.;
    }

    let west_exit = exits.contains(&ExitDirection::West);
    for pos in 0..y_side {
        if west_exit && pos > y_side / 3 && pos < y_side * 2 / 3 {
            v.y += WALL_RADIUS * 2.;
            continue;
        }
        let warmth = a_rng.gen_range(WARMTH_LOW_END..WARMTH_HIGH_END);
        let base_color = a_rng.gen_range(BASE_COLOR_LOW_END..BASE_COLOR_HIGH_END);
        let color = Color::rgb(warmth, base_color, base_color);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::RegularPolygon::new(WALL_RADIUS, 4).into())
                    .into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(v)
                    .with_rotation(Quat::from_rotation_z(a_rng.gen_range(0.0..PI))),
                ..default()
            },
            Collider::new(WALL_RADIUS),
            Wall,
        ));
        v.y += WALL_RADIUS * 2.;
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

    let height = ROOM_HEIGHT;
    let width = ROOM_WIDTH;
    let origin = Vec3::new(-ROOM_WIDTH, ROOM_HEIGHT, 0.);
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
