use crate::collision::Collider;
use crate::movement::{MovingEntityBundle, Velocity};
use crate::LevelState;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const COLOR: Color = Color::rgb(0.9, 0.0, 0.1);
const RADIUS: f32 = 20.;
const SPEED: f32 = 50.;
const STARTING_TRANSLATION: Vec3 = Vec3::new(-300., 400., 0.);

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::One), spawn_enemy.run_if(run_once()))
            .add_systems(Update, passive_motion);
    }
}

fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::RegularPolygon::new(RADIUS, 8).into())
            .into(),
        material: materials.add(ColorMaterial::from(COLOR)),
        transform: Transform::from_translation(STARTING_TRANSLATION),
        ..default()
    };

    commands
        .spawn((
            MovingEntityBundle {
                collider: Collider::new(RADIUS),
                shape,
                velocity: Velocity::new(Vec3::ZERO),
            },
            AnimationPlayer::default(),
            Name::new("player"),
        ))
        .insert(Enemy);
}

fn passive_motion(mut query: Query<(&mut Velocity, &mut Transform), With<Enemy>>, time: Res<Time>) {
    for (mut velocity, mut transform) in query.iter_mut() {
        transform.rotate_z(3. * time.delta_seconds());
        // TODO: how to emulate random orbits? Can't be every update or will just freak out in
        // place! Speed is just about perfect though.
        velocity.value = Vec3::new(0., SPEED, 0.);
    }
}
