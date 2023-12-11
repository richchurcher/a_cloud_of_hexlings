use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    utils::HashMap,
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
    // The things we do because we're outta time... and arrays in WGSL hard?
    #[uniform(2)]
    pub hexling_a: Vec3,
    #[uniform(3)]
    pub hexling_b: Vec3,
    #[uniform(4)]
    pub hexling_c: Vec3,
    #[uniform(5)]
    pub hexling_d: Vec3,
    #[uniform(6)]
    pub hexling_e: Vec3,
    #[uniform(7)]
    pub hexling_f: Vec3,
    #[uniform(8)]
    pub hexling_g: Vec3,
    #[uniform(9)]
    pub hexling_h: Vec3,
    #[uniform(10)]
    pub hexling_i: Vec3,
    #[uniform(11)]
    pub hexling_j: Vec3,
    #[uniform(12)]
    pub hexling_k: Vec3,
    #[uniform(13)]
    pub hexling_l: Vec3,
    #[uniform(14)]
    pub hexling_m: Vec3,
    #[uniform(15)]
    pub hexling_n: Vec3,
    #[uniform(16)]
    pub hexling_o: Vec3,
    #[uniform(17)]
    pub hexling_p: Vec3,
    #[uniform(18)]
    pub hexling_q: Vec3,
    #[uniform(19)]
    pub hexling_r: Vec3,
    #[uniform(20)]
    pub hexling_s: Vec3,
    #[uniform(21)]
    pub hexling_t: Vec3,
    #[uniform(22)]
    pub hexling_u: Vec3,
    #[uniform(23)]
    pub hexling_v: Vec3,
    #[uniform(24)]
    pub hexling_w: Vec3,
    #[uniform(25)]
    pub hexling_x: Vec3,
    #[uniform(26)]
    pub hexling_y: Vec3,
    #[uniform(27)]
    pub hexling_z: Vec3,
}

#[derive(Default, Resource)]
pub struct HexlingFogTracker {
    pub hexling_entity_positions: HashMap<Entity, (Vec3, String)>,
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
            .init_resource::<HexlingFogTracker>()
            .add_systems(OnEnter(GameState::Playing), init)
            .add_systems(Update, update_fog.run_if(in_state(GameState::Playing)));
    }
}

pub fn the_function_that_dare_not_speak_its_name(
    material: &mut FogMaterial,
    pos: &Vec3,
    field: &str,
) {
    match field {
        "a" => material.hexling_a = *pos,
        "b" => material.hexling_b = *pos,
        "c" => material.hexling_c = *pos,
        "d" => material.hexling_d = *pos,
        "e" => material.hexling_e = *pos,
        "f" => material.hexling_f = *pos,
        "g" => material.hexling_g = *pos,
        "h" => material.hexling_h = *pos,
        "i" => material.hexling_i = *pos,
        "j" => material.hexling_j = *pos,
        "k" => material.hexling_k = *pos,
        "l" => material.hexling_l = *pos,
        "m" => material.hexling_m = *pos,
        "n" => material.hexling_n = *pos,
        "o" => material.hexling_o = *pos,
        "p" => material.hexling_p = *pos,
        "q" => material.hexling_q = *pos,
        "r" => material.hexling_r = *pos,
        "s" => material.hexling_s = *pos,
        "t" => material.hexling_t = *pos,
        "u" => material.hexling_u = *pos,
        "v" => material.hexling_v = *pos,
        "w" => material.hexling_w = *pos,
        "x" => material.hexling_x = *pos,
        "y" => material.hexling_y = *pos,
        "z" => material.hexling_z = *pos,
        &_ => todo!(),
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
                light_radius: 500.,
                player_position: Vec3::ZERO,
                hexling_a: Vec3::ZERO,
                hexling_b: Vec3::ZERO,
                hexling_c: Vec3::ZERO,
                hexling_d: Vec3::ZERO,
                hexling_e: Vec3::ZERO,
                hexling_f: Vec3::ZERO,
                hexling_g: Vec3::ZERO,
                hexling_h: Vec3::ZERO,
                hexling_i: Vec3::ZERO,
                hexling_j: Vec3::ZERO,
                hexling_k: Vec3::ZERO,
                hexling_l: Vec3::ZERO,
                hexling_m: Vec3::ZERO,
                hexling_n: Vec3::ZERO,
                hexling_o: Vec3::ZERO,
                hexling_p: Vec3::ZERO,
                hexling_q: Vec3::ZERO,
                hexling_r: Vec3::ZERO,
                hexling_s: Vec3::ZERO,
                hexling_t: Vec3::ZERO,
                hexling_u: Vec3::ZERO,
                hexling_v: Vec3::ZERO,
                hexling_w: Vec3::ZERO,
                hexling_x: Vec3::ZERO,
                hexling_y: Vec3::ZERO,
                hexling_z: Vec3::ZERO,
            }),
            transform: Transform::from_translation(STARTING_TRANSLATION)
                .with_scale(Vec3::splat(3000.))
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
