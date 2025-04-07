use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::*;

macro_rules! vec2 { ($x:expr, $y:expr) => { Vec2 { x: $x, y: $y } }; }

#[derive(Component)]
pub struct PlayerFish;

#[derive(Component)]
pub struct BallFish;

pub fn spawn_playerFish(mut commands: Commands)
{
    commands
        .spawn(RigidBody::Dynamic)
        .insert(PlayerFish)
        .insert(Velocity::default())
        .insert(Collider::capsule(vec2!(0.0, 0.0), vec2!(0.0, 30.0), 30.0))
        .insert(Transform::from_xyz(300.0, 200.0, 0.0))
        .insert(ExternalForce::default())
        .insert(ExternalImpulse::default())
        .insert(Damping {
            linear_damping: 3.0,
            angular_damping: 1.0,
        });
}



pub fn fish_follow_player_system(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &mut ExternalForce), (With<PlayerFish>, Without<Player>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation.truncate();

        for (mut enemy_transform, mut force) in &mut enemy_query {
            let enemy_pos = enemy_transform.translation.truncate();
            let direction = (player_pos - enemy_pos).normalize_or_zero();

            let target_angle = direction.y.atan2(direction.x) - 3.1415926 / 2.0;
            let target_rotation = Quat::from_rotation_z(target_angle);
            let rotation_speed = 5.0;
            enemy_transform.rotation = enemy_transform.rotation.slerp(target_rotation, rotation_speed * time.delta_secs());

            // Ajuste la force appliquée selon le feeling désiré
            let strength = 200_000.0;
            force.force = direction * strength;
        }
    }
}

pub fn fish_follow_ball_system(
    player_query: Query<&Transform, With<Ball>>,
    mut enemy_query: Query<(&mut Transform, &mut ExternalForce), (With<BallFish>, Without<Ball>)>,
    time: Res<Time>,
) {
    if let Ok(ball_transform) = player_query.get_single() {
        let player_pos = ball_transform.translation.truncate();

        for (mut fish_transform, mut force) in &mut enemy_query {
            let enemy_pos = fish_transform.translation.truncate();
            let direction = (player_pos - enemy_pos).normalize_or_zero();

            let target_angle = direction.y.atan2(direction.x) - 3.1415926 / 2.0;
            let target_rotation = Quat::from_rotation_z(target_angle);
            let rotation_speed = 5.0;
            fish_transform.rotation = fish_transform.rotation.slerp(target_rotation, rotation_speed * time.delta_secs());

            // Ajuste la force appliquée selon le feeling désiré
            let strength = 200_000.0;
            force.force = direction * strength;
        }
    }
}