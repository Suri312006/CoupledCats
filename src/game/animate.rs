use rand::prelude::*;
use std::collections::VecDeque;

use bevy::{prelude::*, render::render_resource::Texture};
use color_eyre::eyre::Context;
use log::info;

use super::cat::{CatImageHandles, CatState};

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
pub fn update_texture_from_state(
    textures: Res<CatImageHandles>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut query: Query<(
        &mut TextureAtlas,
        &mut Handle<Image>,
        &mut CatState,
        &mut AnimationIndicies,
    )>,
) {
    // see if there is some new state thats availible to transfer into.

    let (atlas, texture, state, animation_indicies) = query
        .get_single_mut()
        .with_context(|| {
            let err_msg = "failed to update state due to weird query behavior";
            error!(err_msg);
            err_msg
        })
        .unwrap();

    if animation_indicies.last != atlas.index {
        return;
    }

    match state.into_inner() {
        CatState::IDLE => {
            // maybe this isnt the best move, and i should just create a struct that has all the
            // handles on startup, and we can dynamically assign them xd
            *texture.into_inner() = textures.idle.clone();
            *atlas.into_inner() = TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 16),
                    4,
                    1,
                    None,
                    None,
                )),
                index: 0 as usize,
            };
            *animation_indicies.into_inner() = AnimationIndicies { first: 0, last: 3 };
        }

        CatState::LICK => {
            *texture.into_inner() = textures.lick.clone();
            *atlas.into_inner() = TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 16),
                    4,
                    1,
                    None,
                    None,
                )),
                index: 0 as usize,
            };
            *animation_indicies.into_inner() = AnimationIndicies { first: 0, last: 3 };
        }

        CatState::GROOM => {
            *texture.into_inner() = textures.groom.clone();
            *atlas.into_inner() = TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 16),
                    4,
                    1,
                    None,
                    None,
                )),
                index: 0 as usize,
            };
            *animation_indicies.into_inner() = AnimationIndicies { first: 0, last: 3 };
        }

        CatState::JUMP => {
            *texture.into_inner() = textures.jump.clone();
            *atlas.into_inner() = TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 19),
                    7,
                    1,
                    None,
                    None,
                )),
                index: 0 as usize,
            };
            *animation_indicies.into_inner() = AnimationIndicies { first: 0, last: 6 };
        }

        CatState::WALK => {
            *texture.into_inner() = textures.walk.clone();
            *atlas.into_inner() = TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 17),
                    8,
                    1,
                    None,
                    None,
                )),
                index: 0 as usize,
            };
            *animation_indicies.into_inner() = AnimationIndicies { first: 0, last: 7 };
        }

        CatState::SLEEP => {
            *texture.into_inner() = textures.sleep.clone();
            *atlas.into_inner() = TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 15),
                    4,
                    1,
                    None,
                    None,
                )),
                index: 0 as usize,
            };
            *animation_indicies.into_inner() = AnimationIndicies { first: 0, last: 3 };
        }

        CatState::TAP => {
            *texture.into_inner() = textures.tap.clone();
            *atlas.into_inner() = TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 16),
                    6,
                    1,
                    None,
                    None,
                )),
                index: 0 as usize,
            };
            *animation_indicies.into_inner() = AnimationIndicies { first: 0, last: 5 };
        }

        CatState::STRECH => {
            *texture.into_inner() = textures.strech.clone();
            *atlas.into_inner() = TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 16),
                    8,
                    1,
                    None,
                    None,
                )),
                index: 0 as usize,
            };
            *animation_indicies.into_inner() = AnimationIndicies { first: 0, last: 7 };
        }
    }
}

pub fn randomize_state(mut query: Query<&mut CatState>) {
    let mut state = match query.get_single_mut() {
        Ok(state) => state,
        Err(err) => {
            error!("{err:#?}");
            return;
        }
    };

    let mut rng = rand::thread_rng();
    let y = rng.gen_range(0..=7);

    match y {
        0 => *state = CatState::IDLE,
        1 => *state = CatState::LICK,
        2 => *state = CatState::GROOM,
        3 => *state = CatState::JUMP,
        4 => *state = CatState::WALK,
        5 => *state = CatState::SLEEP,
        6 => *state = CatState::TAP,
        7 => *state = CatState::STRECH,
        _ => {
            error!("did not expect rand to generate something thats not 0-7");
            *state = CatState::IDLE;
        }
    }
}
