use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    reflect::TypePath,
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};
use bevy::{
    sprite::{Material2d, Material2dPlugin},
};


pub fn setup_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<CustomMaterial>>, asset_server: Res<AssetServer>)
{
    commands.spawn(Camera2d::default());
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));
    commands
        .spawn(Collider::cuboid(500.0, 25.0))
        .insert(Transform::from_xyz(0.0, -125.0, 0.0));

        // CustomMaterial {
        //     color: LinearRgba::BLUE,
        //     color_texture: Some(asset_server.load("textures/ball.png")),
        // }
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(CustomMaterial {
            // color: LinearRgba::BLUE,
            // color_texture: Some(asset_server.load("textures/ball.png")),
        })),
        Transform::from_xyz(0.0, 0.5, 0.0).with_scale(Vec3 { x: 100.0, y: 100.0, z: 100.0 }),
    ));
}

const SHADER_ASSET_PATH: &str = "shaders/background.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    // #[uniform(0)]
    // color: LinearRgba,
    // #[texture(1)]
    // #[sampler(2)]
    // color_texture: Option<Handle<Image>>,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}