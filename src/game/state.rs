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
        info!("received tick");
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
    let mut queue = match query.get_single_mut() {
        Ok(queue) => queue,
        Err(err) => {
            error!("{err:#?}");
            return;
        }
    };

    if queue.0.len() > 10 {
        return;
    }

    let rand_state = match rand::thread_rng().gen_range(0..=7) {
        0 => CatState::IDLE,
        1 => CatState::LICK,
        2 => CatState::GROOM,
        3 => CatState::JUMP,
        4 => CatState::WALK,
        5 => CatState::SLEEP,
        6 => CatState::TAP,
        7 => CatState::STRECH,
        _ => {
            error!("did not expect rand to generate something thats not 0-7");
            CatState::IDLE
        }
    };

    let curr = queue.0.pop().unwrap_or(CatState::IDLE);
    let rng = rand::thread_rng();

    let next: CatState = match curr {
        CatState::IDLE => {}
        CatState::LICK => {}
        CatState::GROOM => {}
        CatState::JUMP => {}
        CatState::WALK => {}
        CatState::SLEEP => match rng.gen_range(0..100) {
            0..95 => CatState::SLEEP,
            _ => CatState::STRECH,
        },
        CatState::TAP => {
            // nothing should ever lead to here
        }
        CatState::STRECH => {}
    };
}
