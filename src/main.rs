#[path = "systems/hover.rs"] mod hover;
#[path = "systems/player.rs"] mod player;
#[path = "systems/ui.rs"] mod ui;
#[path = "systems/scene.rs"] mod scene;

use bevy::prelude::*;
// use bevy_tweening::TweeningPlugin;
use bevy_rapier2d::prelude::*;
use player::*;
use ui::*;
use scene::*;
use hover::*;

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
        // .add_plugins(TweeningPlugin)
        .add_plugins(default)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_event::<HoveredEvent>()
        .add_systems(Startup, setup_scene)
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, setup_player)
        .add_systems(Update, player_movement)
        .add_systems(Update, update_ball_shadow)
        .add_systems(Update, check_hover_system)
        .run();
}

