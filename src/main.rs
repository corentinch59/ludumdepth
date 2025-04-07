#[path = "systems/player.rs"] mod player;
#[path = "systems/animation.rs"] mod animation;
#[path = "systems/hover.rs"] mod hover;
#[path = "systems/ui.rs"] mod ui;
#[path = "systems/scene.rs"] mod scene;
#[path = "systems/enemy.rs"] mod enemy;

use bevy::prelude::*;
// use bevy_tweening::TweeningPlugin;
use bevy_rapier2d::prelude::*;
use player::*;
use animation::*;
use ui::*;
use scene::*;
use hover::*;
use enemy::*;

fn main() {
    let default = DefaultPlugins
        .build()
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Depths".into(),
                name: Some("Depths".into()),
                fit_canvas_to_parent: true,
                // resolution: (500., 300.).into(),
                // prevent_default_event_handling: false,
                // enabled_buttons: bevy::window::EnabledButtons {
                //     maximize: false,
                //     ..Default::default()
                // },
                ..default()
            }),
            ..default()
        })
        .set(ImagePlugin::default_nearest());

    App::new()
        .insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)))
        .insert_resource(DragState::default())
        .add_event::<HoveredEvent>()
        .add_event::<DragEndedEvent>()
        // .add_plugins(TweeningPlugin)
        .add_plugins(default)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, setup_player)
        .add_systems(Startup, spawn_playerFish)
        .add_systems(Update, player_movement)
        .add_systems(Update, update_ball_shadow)
        .add_systems(Update, check_hover_system)
        .add_systems(Update, click_start_drag_system)
        .add_systems(Update, click_end_drag_system)
        .add_systems(Update, animate_sprite::<PlayerAnimation>)
        .add_systems(Update, animate_sprite::<FishAnimation>)
        .add_systems(Update, apply_drag_impulse_system)
        .add_systems(Update, fish_follow_player_system)
        .add_systems(Update, fish_follow_ball_system)
        .run();
}

