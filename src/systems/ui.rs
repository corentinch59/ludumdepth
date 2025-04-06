
use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>)
{
    let text_font = TextFont {
        font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
        font_size: 25.0,
        ..default()
    };
    commands.spawn(Node::default()).with_children(|parent| {
        parent.spawn((Text::new("Hello from the UI!"), text_font, TextColor(Color::srgb(0.0, 0.0, 0.0))));
    });
}