use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rand::prelude::*;
use rand::prelude::Rng;
use std::f32::consts::PI;

use crate::collision::Collider;
use crate::map::{Source, Wall};
use crate::player::events::SpawnHexlingEvent;

pub struct HexlingPlugin;

impl Plugin for HexlingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hexling_spawner);
    }
}

fn hexling_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ev_spawn_hexling: EventReader<SpawnHexlingEvent>,
    mut query: Query<&mut EntropyComponent<ChaCha8Rng>, With<Source>>,
) {
    let Ok(mut a_rng) = query.get_single_mut() else {
        return;
    };
    if !ev_spawn_hexling.is_empty() {
        ev_spawn_hexling.clear();

        let color = Color::rgb(a_rng.gen_range(0.0..1.0), a_rng.gen_range(0.0..1.0), 0.);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::RegularPolygon::new(10., 5).into()).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(Vec3::ZERO)
                    .with_rotation(Quat::from_rotation_z(a_rng.gen_range(0.0..PI))),
                ..default()
            },
            Collider::new(18.),
            Wall,
        ));
    }
}
