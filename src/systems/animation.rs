use std::collections::HashMap;
use bevy::prelude::*;
use std::hash::Hash;

pub struct AnimationSlice {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct Animator<T> {
    pub current: T,
    pub timer: Timer,
    pub clips: HashMap<T, AnimationSlice>,
}

pub fn animate_sprite<T>(
    time: Res<Time>,
    mut query: Query<(&mut Animator<T>, &mut Sprite)>,
) where T: Send + Sync + 'static + Eq + PartialEq + Hash {
    for (mut manager, mut sprite) in &mut query {
        manager.timer.tick(time.delta());
        if !manager.timer.just_finished() { continue; }
        let Some(atlas) = &mut sprite.texture_atlas else { continue };
        let Some(indices) = manager.clips.get(&manager.current) else { continue };
        let reset = atlas.index < indices.first || atlas.index >= indices.last;
        atlas.index = if reset { indices.first } else {  atlas.index + 1 };
    }
}