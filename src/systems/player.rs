use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::hover::*;

macro_rules! vec2 { ($x:expr, $y:expr) => { Vec2 { x: $x, y: $y } }; }

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct BallShadow;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub struct AnimatedSprite {
    frames: Vec<Handle<Image>>,
    current_frame: usize,
}

const AVATAR_RATIO: f32 = 496.0 / 848.0;
const AVATAR_SIZE: f32 = 350.0;

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>)
{
    let rope = RopeJointBuilder::new(150.0)
        .local_anchor1(vec2!(0.0, 25.0))
        .local_anchor2(vec2!(0.0, 0.0));

    let ball = commands
        .spawn(RigidBody::Dynamic)
        .insert(Ball)
        .insert(Sprite {
            image: asset_server.load("textures/ball.png"),
            custom_size: Some(vec2!(50.0, 50.0)),
            ..default()
        })
        .insert(Collider::ball(25.0))
        .insert(GravityScale(10.0))
        .insert(AdditionalMassProperties::Mass(1000000.0))
        .insert(Transform::from_xyz(10.0, 400.0, 0.0))
        .insert(ExternalForce::default())
        .insert(ExternalImpulse::default())
        .insert(Hoverable)
        .insert(Draggable)
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        })
        .insert(Velocity{
            linvel: vec2!(0.0, 0.0),
            angvel: 0.0
        }).id();

    commands
        .spawn(Sprite {
            image: asset_server.load("textures/ballshadow.png"),
            custom_size: Some(vec2!(50.0, 50.0)),
            ..default()
        })
        .insert(BallShadow);

    let texture = asset_server.load("textures/player.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 5, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mut player_sprite = Sprite::from_atlas_image(
        texture,
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
    );
    player_sprite.custom_size = Some(vec2!(AVATAR_SIZE * AVATAR_RATIO, AVATAR_SIZE));

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Player)
        .insert(Collider::capsule(vec2!(0.0, 0.0), vec2!(0.0, 30.0), 30.0))
        .insert(GravityScale(0.0))
        .insert(ImpulseJoint::new(ball, rope))
        .insert(AdditionalMassProperties::Mass(1.0))
        .insert(ExternalForce::default())
        .insert(ExternalImpulse::default())
        .insert(Damping {
            linear_damping: 3.0,
            angular_damping: 1.0,
        })
        .insert(Velocity{
            linvel: vec2!(0.0, 0.0),
            angvel: 0.0
        })
        .insert(player_sprite)
        .insert(AnimationIndices { first: 1, last: 2 })
        .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .insert(Transform::from_xyz(100.0, 200.0, 0.0));
}


pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut ExternalForce, &mut ExternalImpulse, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    for (mut _force, mut impulse, mut transform) in &mut query {
        let mut direction = Vec2::ZERO;

        if keys.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if keys.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }

        impulse.impulse = direction * 20000.0;

        if direction.length() > 0.1 {
            let target_angle = direction.y.atan2(direction.x) - 3.1415926 / 2.0;
            let target_rotation = Quat::from_rotation_z(target_angle);
            let rotation_speed = 5.0;
            transform.rotation = transform.rotation.slerp(target_rotation, rotation_speed * time.delta_secs());
        }
    }
}
pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

pub fn update_ball_shadow(
    ball: Query<&Transform, With<Ball>>,
    mut ball_shadow: Query<&mut Transform, (With<BallShadow>, Without<Ball>)>,
) {
    let mut shadow_transform = ball_shadow.single_mut();
    let ball_transform = ball.single();
    shadow_transform.translation = ball_transform.translation;
}

pub fn apply_drag_impulse_system(
    mut impulses: Query<&mut ExternalImpulse>,
    mut events: EventReader<DragEndedEvent>,
) {
    for event in events.read() {
        if let Ok(mut impulse) = impulses.get_mut(event.entity) {
            // L'impulsion est proportionnelle à la distance (delta) du drag
            impulse.impulse = -event.delta * 1500000.0; // Ajuste le facteur selon le feeling
            println!("Impulse applied to {:?}: {:?}", event.entity, impulse.impulse);
        }
    }
}
