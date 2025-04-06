use bevy::prelude::*;

// HOVER 

#[derive(Component)]
pub struct Hoverable;

#[derive(Event)]
pub struct HoveredEvent
{
    pub entity: Entity,
}

#[derive(Component)]
pub struct Draggable;

#[derive(Resource, Default)]
pub struct DragState {
    pub active_entity: Option<Entity>,
    pub drag_start: Option<Vec2>,
}

#[derive(Event)]
pub struct DragEndedEvent {
    pub entity: Entity,
    pub start: Vec2,
    pub end: Vec2,
    pub delta: Vec2,
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

// DRAG 

pub fn click_start_drag_system(
    buttons: Res<ButtonInput<MouseButton>>,
    mut drag_state: ResMut<DragState>,
    mut events: EventReader<HoveredEvent>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    query: Query<Entity, With<Draggable>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = cursor_world_position(windows, camera_q) {
            for HoveredEvent { entity } in events.read() {
                if query.get(*entity).is_ok() {
                    drag_state.active_entity = Some(*entity);
                    drag_state.drag_start = Some(cursor_pos);
                }
            }
        }
    }
}

pub fn click_end_drag_system(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut drag_state: ResMut<DragState>,
    mut drag_ended_writer: EventWriter<DragEndedEvent>,
) {
    if buttons.just_released(MouseButton::Left) {
        if let (Some(entity), Some(start_pos)) = (
            drag_state.active_entity.take(),
            drag_state.drag_start.take(),
        ) {
            if let Some(end_pos) = cursor_world_position(windows, camera_q) {
                let delta = end_pos - start_pos;
                drag_ended_writer.send(DragEndedEvent {
                    entity,
                    start: start_pos,
                    end: end_pos,
                    delta,
                });
            }
        }
    }
}