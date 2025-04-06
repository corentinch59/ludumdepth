use bevy::prelude::*;

#[derive(Component)]
pub struct Hoverable;

#[derive(Event)]
pub struct HoveredEvent
{
    pub entity: Entity,
}

fn cursor_world_position(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let (camera, camera_transform) = camera_q.single();
    let window = windows.single();

    window.cursor_position().and_then(|cursor_pos| {
        camera
            .viewport_to_world_2d(camera_transform, cursor_pos)
            .ok()
    })
}
    
pub fn check_hover_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    cursor_pos: Res<ButtonInput<MouseButton>>,
    mut events: EventWriter<HoveredEvent>,
    query: Query<(&GlobalTransform, &Sprite, Entity), With<Hoverable>>,
) {
    if let Some(cursor_world) = cursor_world_position(windows, camera_q) {
        for (transform, sprite, entity) in &query {
            let scale = transform.scale().truncate();
            let size = sprite.custom_size.unwrap_or(Vec2::ONE) * scale;
            let position = transform.translation().truncate();

            let half_size = size / 2.0;
            let min = position - half_size;
            let max = position + half_size;

            if (min.x..=max.x).contains(&cursor_world.x)
                && (min.y..=max.y).contains(&cursor_world.y)
            {
                events.send(HoveredEvent { entity });
            }
        }
    }
}