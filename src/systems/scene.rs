use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn setup_scene(mut commands: Commands)
{
    commands.spawn(Camera2d::default());
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));
    commands
        .spawn(Collider::cuboid(500.0, 25.0))
        .insert(Transform::from_xyz(0.0, -125.0, 0.0));
}