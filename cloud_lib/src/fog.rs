use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

use std::f32::consts::PI;
use type_uuid::TypeUuid;

use crate::{
    player::{Player, STARTING_TRANSLATION},
    GameState,
};

#[derive(AsBindGroup, Asset, Clone, Debug, TypePath, TypeUuid)]
#[uuid = "c4a06f14-bd8d-4949-bfdb-b84719933e76"]
pub struct FogMaterial {
    pub alpha_mode: AlphaMode,
    #[uniform(0)]
    pub light_radius: f32,
    #[uniform(1)]
    pub player_position: Vec3,
}

impl Material2d for FogMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fog.wgsl".into()
    }
}

#[derive(Component)]
pub struct Fog;

pub struct FogPlugin;

impl Plugin for FogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<FogMaterial>::default())
            .add_systems(OnEnter(GameState::Playing), init)
            .add_systems(Update, update_fog.run_if(in_state(GameState::Playing)));
    }
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FogMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle::<FogMaterial> {
            mesh: meshes.add(shape::RegularPolygon::new(1., 4).into()).into(),
            material: materials.add(FogMaterial {
                alpha_mode: AlphaMode::Blend,
                light_radius: 0.3,
                player_position: Vec3::ZERO,
            }),
            transform: Transform::from_translation(STARTING_TRANSLATION)
                .with_scale(Vec3::splat(5000.))
                .with_rotation(Quat::from_rotation_z(PI / 4.)),
            ..Default::default()
        })
        .insert(Fog);
}

fn update_fog(
    mut handle: Query<(&Handle<FogMaterial>, &mut Transform), With<Fog>>,
    mut materials: ResMut<Assets<FogMaterial>>,
    query: Query<&Transform, (With<Player>, Without<Fog>)>,
) {
    let Ok(player_transform) = query.get_single() else {
        return;
    };
    let Ok((fog_handle, mut fog_transform)) = handle.get_single_mut() else {
        return;
    };
    let fog_material = materials.get_mut(fog_handle).unwrap();
    fog_material.player_position = player_transform.translation;

    // Keep the mesh centered on the player
    fog_transform.translation = Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y,
        1.,
    );
}
