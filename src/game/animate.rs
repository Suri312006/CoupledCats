use color_eyre::eyre::Context;
use rand::prelude::*;

use bevy::prelude::*;

use super::{
    cat::{CatImageHandles, StateQueue},
    state::CatState,
};

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndicies {
    pub first: usize,
    pub last: usize,
}

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

// this needs to run off of a signal

pub fn randomize_state(mut query: Query<&mut StateQueue<CatState>>) {
    let mut state = match query.get_single_mut() {
        Ok(state) => state,
        Err(err) => {
            error!("{err:#?}");
            return;
        }
    };

    let mut rng = rand::thread_rng();
    let y = rng.gen_range(0..=7);

    if state.0.len() > 200 {
        return;
    }

    match y {
        0 => state.0.push(CatState::IDLE),
        1 => state.0.push(CatState::LICK),
        2 => state.0.push(CatState::GROOM),
        3 => state.0.push(CatState::JUMP),
        4 => state.0.push(CatState::WALK),
        5 => state.0.push(CatState::SLEEP),
        6 => state.0.push(CatState::TAP),
        7 => state.0.push(CatState::STRECH),
        // 1 => *state = CatState::LICK,
        // 2 => *state = CatState::GROOM,
        // 3 => *state = CatState::JUMP,
        // 4 => *state = CatState::WALK,
        // 5 => *state = CatState::SLEEP,
        // 6 => *state = CatState::TAP,
        // 7 => *state = CatState::STRECH,
        _ => {
            error!("did not expect rand to generate something thats not 0-7");

            state.0.push(CatState::STRECH);
        }
    }
}
