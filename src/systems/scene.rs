use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;
use crate::vec2;

const GROUND_SIZE: f32 = 750.0;
const GROUND_RATIO: f32 = 1727.0 / 599.0;
pub fn setup_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<CustomMaterial>>, asset_server: Res<AssetServer>)
{
    commands.spawn(Camera2d::default());
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));
    commands
        .spawn(Collider::cuboid(500.0, 25.0))
        .insert(Transform::from_xyz(0.0, -25.0, 0.0))
        .insert(CollisionGroups::new(
            Group::GROUP_1,
            Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_3 | Group::GROUP_4,
        ));
    commands
        .spawn(Collider::cuboid(25.0, 10000.0))
        .insert(Transform::from_xyz(-400.0, 4000.0, 0.0))
        .insert(CollisionGroups::new(
            Group::GROUP_1,
            Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_3 | Group::GROUP_4,
        ));
    commands
        .spawn(Collider::cuboid(25.0, 10000.0))
        .insert(Transform::from_xyz(400.0, 4000.0, 0.0))
        .insert(CollisionGroups::new(
            Group::GROUP_1,
            Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_3 | Group::GROUP_4,
        ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(CustomMaterial {})),
        Transform::from_xyz(0.0, -0.5, 0.0).with_scale(Vec3 { x: 100.0, y: 100.0, z: 100.0 }),
    ));
    
    commands
        .spawn(Sprite {
            image: asset_server.load("textures/ground.png"),
            custom_size: Some(vec2!(GROUND_RATIO * GROUND_SIZE, GROUND_SIZE)),
            ..default()
        });
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