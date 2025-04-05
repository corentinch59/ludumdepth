use bevy::{prelude::*, window::PrimaryWindow};
use bevy_tweening::TweeningPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TweeningPlugin)
        .add_systems(Startup, (set_window_name, setup).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
    )
{
    let text_font = TextFont 
    {
        font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
        font_size: 25.0,
        ..default()
    };

    // --- Caméra 3D ---
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y)));

    // --- Une light ---
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));

    // --- UI ---
    commands.spawn(Node::default()).with_children(|parent| {
        parent.spawn((Text::new("Hello from the UI!"), text_font));
    });

    // commands.spawn((SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/result.gltf"))), Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3 { x: 0.1, y: 0.1, z: 0.1 })));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}

fn set_window_name(mut window_query: Query<&mut Window, With<PrimaryWindow>>)
{
    if let Ok(mut window) = window_query.get_single_mut()
    {
        window.title = "Depths".to_string();
    }
}