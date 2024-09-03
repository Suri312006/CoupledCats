use super::{animate::AnimationIndicies, SpriteTick};
use bevy::prelude::*;
use color_eyre::eyre::{Context, ContextCompat};
use log::info;
use rand::Rng;

#[derive(Component, Debug)]
pub enum CatState {
    IDLE,
    LICK,
    GROOM,
    WALK,
    SLEEP,
    TAP,
    JUMP,
    STRECH,
}

#[derive(Component, Default)]
pub struct StateQueue<T>(pub Vec<T>);

pub fn update_state_from_queue(
    mut ev_sprite_tick: EventReader<SpriteTick>,
    mut query: Query<(
        &mut TextureAtlas,
        &mut CatState,
        &mut StateQueue<CatState>,
        &mut AnimationIndicies,
    )>,
) {
    let (atlas, state, mut queue, animation_indicies) = query
        .get_single_mut()
        .with_context(|| {
            let err_msg = "failed to update state due to weird query behavior";
            error!(err_msg);
            err_msg
        })
        .unwrap();

    let state = state.into_inner();

    for _tick in ev_sprite_tick.read() {
        info!("received");
        *state = queue
            .0
            .pop()
            .with_context(|| {
                let err = "Queue for cat is empty";
                error!(err);
                err
            })
            .unwrap();
        info!("what the fuck {:#?}", &state);
    }

    // if animation_indicies.last != atlas.index {
    //     return;
    // } else {
    // }
}

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

    if state.0.len() > 10 {
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
        _ => {
            error!("did not expect rand to generate something thats not 0-7");

            state.0.push(CatState::STRECH);
        }
    }
}
