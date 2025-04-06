use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;
use crate::hover::*;

macro_rules! vec2 { ($x:expr, $y:expr) => { Vec2 { x: $x, y: $y } }; }

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct BallShadow;

#[derive(Component)]
pub struct Ball;

#[derive(Eq, Hash, PartialEq)]
enum Animation {
    Idle,
    Swiming,
    Trackted,
}

struct AnimationClip {
    first: usize,
    last: usize,
}

#[derive(Component)]
pub struct AnimationManager {
    current: Animation,
    clips: HashMap<Animation, AnimationClip>,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

const HEIGHT: u32 = 848;
const WIDTH: u32 = 496;
const AVATAR_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;
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
    let layout = TextureAtlasLayout::from_grid(UVec2 { x: WIDTH, y: HEIGHT }, 5, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mut player_sprite = Sprite::from_atlas_image(
        texture,
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 1,
        },
    );
    player_sprite.custom_size = Some(vec2!(AVATAR_SIZE * AVATAR_RATIO, AVATAR_SIZE));

    let mut clips = HashMap::new();
    clips.insert(Animation::Idle, AnimationClip { first: 0, last: 1 });
    clips.insert(Animation::Trackted, AnimationClip { first: 2, last: 2 });
    clips.insert(Animation::Swiming, AnimationClip { first: 3, last: 5 });

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
        .insert(AnimationManager {
            current: Animation::Idle,
            clips: clips,
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .insert(Transform::from_xyz(100.0, 200.0, 0.0));
}


pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut ExternalForce, &mut ExternalImpulse, &mut Transform, &mut AnimationManager), With<Player>>,
    time: Res<Time>,
    ball_query: Query<&Velocity, With<Ball>>,
) {
    let ball = ball_query.single();
    for (mut _force, mut impulse, mut transform, mut manager) in &mut query {
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
            manager.current = Animation::Swiming;
            let target_angle = direction.y.atan2(direction.x) - 3.1415926 / 2.0;
            let target_rotation = Quat::from_rotation_z(target_angle);
            let rotation_speed = 5.0;
            transform.rotation = transform.rotation.slerp(target_rotation, rotation_speed * time.delta_secs());
        } else if ball.linvel.length() > 0.2 {
            manager.current = Animation::Trackted;
        } else {
            manager.current = Animation::Idle;
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationManager, &mut AnimationTimer, &mut Sprite)>,
) {
    for (manager, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if let Some(indices) = manager.clips.get(&manager.current) {
                    atlas.index = if atlas.index < indices.first || atlas.index >= indices.last {
                        indices.first
                    } else {
                        atlas.index + 1
                    };
                }

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
