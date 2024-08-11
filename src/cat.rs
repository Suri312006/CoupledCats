use bevy::prelude::*;

use crate::{animate::AnimationIndicies, AnimationTimer};

#[derive(Component)]
pub struct Cat {}

#[derive(Bundle)]
struct CatBundle {
    velocity: Velocity,
    bounds: Bounds,
    sprite: SpriteBundle,
    texture_atlas: TextureAtlas,
    animation_timer: AnimationTimer,
    animation_indicies: AnimationIndicies,
}

#[derive(Component, Default)]
pub struct Velocity(pub IVec2);

#[derive(Component, Default)]
pub struct Bounds(pub UVec2);
// cat has sprite stuff
impl Cat {
    pub fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        //TODO: bind it to one monitor
        // okay so we jsut have to cap these to the display size
        // https://docs.rs/winit/0.26.1/winit/event_loop/struct.EventLoopWindowTarget.html#method.available_monitors
        // TODO: use the link above to set proper sizing constraints
        // for display in DisplayInfo::all().unwrap() {
        //     cat_state.bounds.x += display.width;
        //     // okay so this might be weird, ideally we would want it to choose the max one?
        //     cat_state.bounds.y = display.height;
        // }

        let texture = asset_server.load("fox-run.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
        let texture_atlas_layout = texture_atlas_layout.add(layout);
        let animation_indicies = AnimationIndicies { first: 1, last: 5 };

        commands.spawn(Camera2dBundle::default());
        commands.spawn(CatBundle {
            velocity: Velocity(IVec2::new(0, 0)),
            bounds: Bounds(UVec2::new(1920 - 300, 1080)),
            sprite: SpriteBundle {
                transform: Transform::from_scale(Vec3::splat(5.0)),
                texture,
                ..default()
            },
            texture_atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indicies.first,
            },
            animation_indicies,
            animation_timer: AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        });
    }
}
