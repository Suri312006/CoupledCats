use bevy::prelude::*;

use super::Cat;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndicies {
    pub first: usize,
    pub last: usize,
}

#[derive(Event)]
pub struct SpriteTick {}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndicies, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

pub fn cat_sprite_tick(
    mut sprite_tick: EventWriter<SpriteTick>,
    mut query: Query<(&AnimationIndicies, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    let (indices, mut timer, mut atlas) = query.get_single_mut().expect("wanted to find cat");

    if indices.last == atlas.index {
        sprite_tick.send(SpriteTick {});
    }
}
