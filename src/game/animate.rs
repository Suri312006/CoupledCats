use bevy::{prelude::*, render::render_resource::Texture};
use color_eyre::eyre::Context;

use super::cat::CatState;

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
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut query: Query<(&mut TextureAtlas, &mut Handle<Image>, &mut CatState)>,
) {
    let (mut atlas, mut texture, state) = query
        .get_single_mut()
        .with_context(|| {
            let err_msg = "failed to update state due to weird query behavior";
            error!(err_msg);
            err_msg
        })
        .unwrap();

    match state.into_inner() {
        CatState::IDLE => {
            // maybe this isnt the best move, and i should just create a struct that has all the
            // handles on startup, and we can dynamically assign them xd
            *texture.into_inner() = asset_server.load::<Image>("cat/groom.png");
            *atlas.into_inner() = TextureAtlas {
                layout: texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 21),
                    4,
                    1,
                    None,
                    None,
                )),
                index: 0 as usize,
            }
        }
        _ => {}
    }
}
