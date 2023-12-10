use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::{AsBindGroup, PrimitiveTopology, ShaderRef},
    },
};

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct FogMaterial {
    // #[uniform(0)]
    // color: Color,
    // color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

impl Material for FogMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fog.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

pub struct FogPlugin;

impl Plugin for FogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<FogMaterial>::default())
            .add_systems(Startup, init);
    }
}

const MASK_WIDTH: f32 = 100.;
const MASK_HEIGHT: f32 = 100.;

fn init(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FogMaterial>>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(vec![
                [0.0, 0.0, 1.0],
                [MASK_WIDTH, 0.0, 0.0],
                [MASK_WIDTH, MASK_HEIGHT, 0.0],
                [0.0, MASK_HEIGHT, 0.0],
            ]),
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float32x3(vec![
                [0., 0., 1.],
                [0., 0., 1.],
                [0., 0., 1.],
                [0., 0., 1.],
            ]),
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            VertexAttributeValues::Float32x2(vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]]),
        )
        .with_indices(Some(Indices::U32(vec![0, 1, 2, 2, 3, 0])));
    let handle = meshes.add(mesh);
    commands.spawn(MaterialMeshBundle::<FogMaterial> {
        mesh: handle,
        material: materials.add(FogMaterial {
            alpha_mode: AlphaMode::Blend,
        }),
        ..Default::default()
    });
}
