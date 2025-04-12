use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::hover::*;
use crate::vec2;
use crate::animation::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Ring;

#[derive(Eq, Hash, PartialEq)]
pub enum PlayerAnimation {
    Idle,
    Swimming,
    Trackted,
}

const HEIGHT: u32 = 848;
const WIDTH: u32 = 496;
const AVATAR_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;
const AVATAR_SIZE: f32 = 350.0;

const NUM_OF_RINGS: usize = 0;
const STEP_ROPE_DISTANCE: f32 = 100.0;
const EDGE_DISTANCE: f32 = 11.0;
const RING_RATIO: f32 = 1.0;
const RING_SIZE: f32 = 20.0;

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
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
    clips.insert(PlayerAnimation::Idle, AnimationSlice { first: 0, last: 1 });
    clips.insert(PlayerAnimation::Trackted, AnimationSlice { first: 2, last: 2 });
    clips.insert(PlayerAnimation::Swimming, AnimationSlice { first: 3, last: 5 });

    let mut previous_entity =  commands
        .spawn(RigidBody::Dynamic)
        .insert(Player)
        .insert(Collider::capsule(vec2!(0.0, -10.0), vec2!(0.0, 45.0), 30.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(GravityScale(0.0))
        .insert(AdditionalMassProperties::Mass(1.0))
        .insert(ExternalForce::default())
        .insert(ExternalImpulse::default())
        .insert(CollisionGroups::new(
            Group::GROUP_2,
            Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_4,
        ))
        .insert(Damping {
            linear_damping: 3.0,
            angular_damping: 1.0,
        })
        .insert(Velocity {
            linvel: vec2!(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(player_sprite)
        .insert(Animator {
            current: PlayerAnimation::Idle,
            clips: clips,
            timer: Timer::from_seconds(0.3, TimerMode::Repeating),
        })
        .insert(Transform::from_xyz(100.0, 50.0, 0.0))
        .id();

    for i in 0..NUM_OF_RINGS {
        let ring_texture = asset_server.load("textures/ring.png");
        
        let rope = RopeJointBuilder::new(STEP_ROPE_DISTANCE)
            .local_anchor1(vec2!(if i == 0 { 0.0 } else { EDGE_DISTANCE }, 0.0))
            .local_anchor2(vec2!(-EDGE_DISTANCE, 0.0));

        previous_entity = commands
            .spawn(RigidBody::Dynamic)
            .insert(Ring)
            .insert(Sprite {
                image: ring_texture,
                custom_size: Some(vec2!(RING_SIZE * RING_RATIO, RING_SIZE * RING_RATIO)),
                ..default()
            })
            .insert(CollisionGroups::new(
                Group::GROUP_3,
                Group::GROUP_1 | Group::GROUP_4,
            ))
            .insert(GravityScale(1.0))
            .insert(AdditionalMassProperties::Mass(1000.0))
            .insert(Collider::capsule(vec2!(0.0, 0.0), vec2!(0.0, 0.0), RING_SIZE / 3.0))
            .insert(Transform::from_xyz(
                // (NUM_OF_RINGS - 1 - i) as f32 * (STEP_ROPE_DISTANCE + EDGE_DISTANCE * 2.0) * 0.5,
                0.0,
                50.0,
                0.0,
            ))
            // .insert(ImpulseJoint::new(previous_entity, rope))
            .insert_if(ImpulseJoint::new(previous_entity, rope), || { i > 0 })
            .id();
    }

    let rope = RopeJointBuilder::new(STEP_ROPE_DISTANCE)
        // .local_anchor1(vec2!(EDGE_DISTANCE, 0.0))
        .local_anchor1(vec2!(0.0, 0.0))
        .local_anchor2(vec2!(0.0, 25.0));

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Ball)
        .insert(Sprite {
            image: asset_server.load("textures/ball.png"),
            custom_size: Some(vec2!(50.0, 50.0)),
            ..default()
        })
        .insert(CollisionGroups::new(
            Group::GROUP_2,
            Group::GROUP_1 | Group::GROUP_2,
        ))
        .insert(Collider::ball(25.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(GravityScale(10.0))
        .insert(AdditionalMassProperties::Mass(1_000_000.0))
        .insert(Transform::from_xyz(0.0, 25.0, 0.0))
        .insert(ExternalForce::default())
        .insert(ExternalImpulse::default())
        .insert(Hoverable)
        .insert(Draggable)
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        })
        .insert(ImpulseJoint::new(previous_entity, rope))
        .insert(Velocity {
            linvel: vec2!(0.0, 0.0),
            angvel: 0.0,
        });
}

pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut ExternalForce, &mut ExternalImpulse, &mut Transform, &mut Animator<PlayerAnimation>, &Velocity), With<Player>>,
    time: Res<Time>,
) {
    for (mut _force, mut impulse, mut transform, mut manager, velocity) in &mut query {
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

        impulse.impulse = direction * 20_000.0;

        if direction.length() > 0.1 {
            manager.current = PlayerAnimation::Swimming;
            let target_angle = direction.y.atan2(direction.x) - 3.1415926 / 2.0;
            let target_rotation = Quat::from_rotation_z(target_angle);
            let rotation_speed = 5.0;
            transform.rotation = transform.rotation.slerp(target_rotation, rotation_speed * time.delta_secs());
        } else if velocity.linvel.length() > 20.0 {
            manager.current = PlayerAnimation::Trackted;
        } else {
            manager.current = PlayerAnimation::Idle;
        }
    }
}

pub fn apply_drag_impulse_system(
    mut impulses: Query<(&mut ExternalImpulse, &Velocity)>,
    mut events: EventReader<DragEndedEvent>,
) {
    for event in events.read() {
        let Ok((mut impulse, velocity)) = impulses.get_mut(event.entity) else { continue };
        // if velocity.linvel.length_squared() > 60.0 { continue }
        let force = -event.delta * 1_500_000.0;
        impulse.impulse = force;
    }
}
