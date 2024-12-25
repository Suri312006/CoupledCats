use bevy::prelude::*;
use color_eyre::eyre::Context;

use crate::game::cat::*;

use super::{animate::AnimationIndices, behavior::CatState};

pub fn update_texture_from_state(
    textures: Res<CatImageHandles>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut query: Query<(&mut Sprite, &CatState, &mut AnimationIndices)>,
) {
    // see if there is some new state thats available to transfer into.

    let (mut sprite, state, animation_indices) = query
        .get_single_mut()
        .with_context(|| {
            let err_msg = "failed to update state due to weird query behavior";
            error!(err_msg);
            err_msg
        })
        .unwrap();

    let atlas = sprite
        .texture_atlas
        .as_mut()
        .expect("Expecting Sprite to have Texture Atlas");

    if animation_indices.last != atlas.index {
        return;
    }

    match state {
        CatState::IDLE => {
            // maybe this isnt the best move, and i should just create a struct that has all the
            // handles on startup, and we can dynamically assign them xd
            sprite.image = textures.idle.clone();

            *sprite
                .texture_atlas
                .as_mut()
                .expect("Expecting Sprite to have Texture Atlas") = TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 16),
                    4,
                    1,
                    None,
                    None,
                )),
                index: 0 as usize,
            };
            *animation_indices.into_inner() = AnimationIndices { first: 0, last: 3 };
        }

        CatState::LICK => {
            sprite.image = textures.lick.clone();
            *sprite
                .texture_atlas
                .as_mut()
                .expect("Expecting Sprite to have Texture Atlas") = TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 16),
                    4,
                    1,
                    None,
                    None,
                )),
                index: 0 as usize,
            };
            *animation_indices.into_inner() = AnimationIndices { first: 0, last: 3 };
        }

        CatState::GROOM => {
            sprite.image = textures.groom.clone();
            *sprite
                .texture_atlas
                .as_mut()
                .expect("Expecting Sprite to have Texture Atlas") = TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 16),
                    4,
                    1,
                    None,
                    None,
                )),
                index: 0 as usize,
            };
            *animation_indices.into_inner() = AnimationIndices { first: 0, last: 3 };
        }

        CatState::JUMP => {
            sprite.image = textures.jump.clone();
            *sprite
                .texture_atlas
                .as_mut()
                .expect("Expecting Sprite to have Texture Atlas") = TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 19),
                    7,
                    1,
                    None,
                    None,
                )),
                index: 0 as usize,
            };
            *animation_indices.into_inner() = AnimationIndices { first: 0, last: 6 };
        }

        CatState::WALK => {
            sprite.image = textures.walk.clone();
            *sprite
                .texture_atlas
                .as_mut()
                .expect("Expecting Sprite to have Texture Atlas") = TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 17),
                    8,
                    1,
                    None,
                    None,
                )),
                index: 0 as usize,
            };
            *animation_indices.into_inner() = AnimationIndices { first: 0, last: 7 };
        }

        CatState::SLEEP => {
            sprite.image = textures.sleep.clone();
            *sprite
                .texture_atlas
                .as_mut()
                .expect("Expecting Sprite to have Texture Atlas") = TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 15),
                    4,
                    1,
                    None,
                    None,
                )),
                index: 0 as usize,
            };
            *animation_indices.into_inner() = AnimationIndices { first: 0, last: 3 };
        }

        CatState::TAP => {
            sprite.image = textures.tap.clone();
            *sprite
                .texture_atlas
                .as_mut()
                .expect("Expecting Sprite to have Texture Atlas") = TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 16),
                    6,
                    1,
                    None,
                    None,
                )),
                index: 0 as usize,
            };
            *animation_indices.into_inner() = AnimationIndices { first: 0, last: 5 };
        }

        CatState::STRETCH => {
            sprite.image = textures.stretch.clone();
            *sprite
                .texture_atlas
                .as_mut()
                .expect("Expecting Sprite to have Texture Atlas") = TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 16),
                    8,
                    1,
                    None,
                    None,
                )),
                index: 0 as usize,
            };
            *animation_indices.into_inner() = AnimationIndices { first: 0, last: 7 };
        }
    }
}
