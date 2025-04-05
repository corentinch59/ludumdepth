use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands : Commands)
{
    commands.spawn((Person, Name("Corentin Chrisment".to_string())));
    commands.spawn((Person, Name("Boboch D Brew".to_string())));
}

fn hello_world()
{
    println!("Hello world");
}

fn greet_people(query : Query<&Name, With<Person>>)
{
    for name in &query
    {
        println!("Hello {}", name.0);
    }
}

fn set_window_name(mut window_query: Query<&mut Window, With<PrimaryWindow>>)
{
    if let Ok(mut window) = window_query.get_single_mut()
    {
        window.title = "Depths".to_string();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (add_people, set_window_name))
        .add_systems(Update, (hello_world, greet_people).chain())
        .run();
}