use animate::{animate_sprite, cat_sprite_tick, SpriteTick};
use bevy::prelude::*;
use movement::move_window;
use network::{init_network_systems, NetworkEvent};
use state::{randomize_state, update_state_from_queue};
use texture::update_texture_from_state;

use super::cat::Cat;

pub(super) mod animate;
pub(super) mod movement;
pub(super) mod network;
pub(super) mod state;
pub(super) mod texture;

pub fn init_systems(app: &mut App) {
    app
        // startup
        .add_systems(Startup, Cat::setup)
        //movement
        // .add_systems(Update, move_window)
        //animate
        .add_event::<SpriteTick>()
        .add_event::<NetworkEvent>()
        .add_systems(Update, (animate_sprite, cat_sprite_tick).chain())
        //state
        .add_systems(
            Update,
            (
                update_state_from_queue,
                update_texture_from_state,
                move_window,
            )
                .chain(),
        ) //// not remove this chain
        //texture
        .add_systems(Update, randomize_state);

    init_network_systems(app);
}
