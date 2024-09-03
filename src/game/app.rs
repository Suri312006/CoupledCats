use bevy::{
    prelude::*,
    window::{Cursor, WindowLevel, WindowResolution},
};

use super::{
    animate::animate_sprite,
    cat_sprite_tick,
    movement::move_window,
    state::{randomize_state, update_state_from_queue},
    texture::update_texture_from_state,
    Cat, SpriteTick,
};

pub struct CoupledCats;

impl CoupledCats {
    pub fn run(mut app: App) {
        let trans_window = Window {
            transparent: true,
            decorations: false,
            window_level: WindowLevel::AlwaysOnTop,
            resizable: false,
            resolution: WindowResolution::new(200.0, 200.0).with_scale_factor_override(1.0),
            position: WindowPosition::Centered(MonitorSelection::Current),
            cursor: Cursor {
                hit_test: false,
                ..default()
            },
            ..default()
        };

        app.insert_resource(ClearColor(Color::NONE))
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(trans_window),
                        ..default()
                    })
                    .set(ImagePlugin::default_nearest()), // needed for clear sprites,
            )
            // startup
            .add_systems(Startup, Cat::setup)
            //movement
            // .add_systems(Update, move_window)
            //animate
            .add_event::<SpriteTick>()
            .add_systems(
                Update,
                (animate_sprite, cat_sprite_tick, move_window).chain(),
            )
            //state
            .add_systems(
                Update,
                (update_state_from_queue, update_texture_from_state).chain(),
            ) //// not remove this chain
            //texture
            .add_systems(Update, randomize_state)
            .run();
    }
}
