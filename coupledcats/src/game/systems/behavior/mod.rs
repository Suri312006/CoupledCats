// /// update loop for the behavior simulator, basically what runs every tick
// pub fn update() {}

use bevy::prelude::*;
use color_eyre::eyre::{Context, ContextCompat};
use rand::Rng;
use std::collections::VecDeque;

use super::animate::{AnimationIndices, SpriteTick};
#[derive(Component, Debug)]
pub enum CatState {
    IDLE,
    LICK,
    GROOM,
    WALK,
    SLEEP,
    TAP,
    JUMP,
    STRETCH,
}

#[derive(Component, Default)]
pub struct StateQueue<T>(pub VecDeque<T>);

pub fn update_state_from_queue(
    mut ev_sprite_tick: EventReader<SpriteTick>,
    mut query: Query<(&mut CatState, &mut StateQueue<CatState>)>,
) {
    let (state, mut queue) = query
        .get_single_mut()
        .with_context(|| {
            let err_msg = "failed to update state due to weird query behavior";
            error!(err_msg);
            err_msg
        })
        .unwrap();

    let state = state.into_inner();

    for _tick in ev_sprite_tick.read() {
        trace!("received tick");
        *state = queue
            .0
            .pop_front()
            .with_context(|| {
                let err = "Queue for cat is empty";
                error!(err);
                err
            })
            .unwrap();
        debug!("Current State {:#?}", &state);
    }
}

pub fn randomize_state(mut query: Query<&mut StateQueue<CatState>>) {
    let mut queue = match query.get_single_mut() {
        Ok(queue) => queue,
        Err(err) => {
            error!("{err:#?}");
            return;
        }
    };

    if queue.0.len() > 2 {
        return;
    }

    let curr = queue.0.pop_back().unwrap_or(CatState::IDLE);
    let mut rng = rand::thread_rng();

    let next: CatState = match curr {
        CatState::IDLE => match rng.gen_range(0..100) {
            0..90 => CatState::IDLE,
            90..93 => CatState::GROOM,
            93..97 => CatState::WALK,
            _ => CatState::SLEEP,
        },
        CatState::LICK => match rng.gen_range(0..100) {
            0..50 => CatState::LICK,
            0..75 => CatState::GROOM,
            _ => CatState::IDLE,
        },
        CatState::GROOM => match rng.gen_range(0..100) {
            0..25 => CatState::GROOM,
            25..50 => CatState::LICK,
            _ => CatState::WALK,
        },
        CatState::JUMP => match rng.gen_range(0..100) {
            _ => CatState::WALK,
        },
        CatState::WALK => match rng.gen_range(0..100) {
            0..75 => CatState::WALK,
            75..85 => CatState::JUMP,
            _ => CatState::IDLE,
        },
        CatState::SLEEP => match rng.gen_range(0..100) {
            0..99 => CatState::SLEEP,
            _ => CatState::STRETCH,
        },
        CatState::TAP => {
            // nothing should ever lead to here
            CatState::IDLE
        }
        CatState::STRETCH => CatState::IDLE,
    };

    queue.as_mut().0.push_back(curr);
    queue.as_mut().0.push_back(next);
}
