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
    pub clips: HashMap<T, AnimationSlice>,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn animate_sprite<T>(
    time: Res<Time>,
    mut query: Query<(&Animator<T>, &mut AnimationTimer, &mut Sprite)>,
) where T: Send + Sync + 'static + Eq + PartialEq + Hash {
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