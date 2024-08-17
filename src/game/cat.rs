use bevy::prelude::*;

use super::animate::{AnimationIndicies, AnimationTimer};

#[derive(Component)]
pub struct Cat {}

#[derive(Bundle)]
struct CatBundle {
    state: CatState,
    velocity: Velocity,
    bounds: Bounds,
    sprite: SpriteBundle,
    texture_atlas: TextureAtlas,
    animation_timer: AnimationTimer,
    animation_indicies: AnimationIndicies,
}

#[derive(Component)]
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

        // this was for fox run
        // let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
        // let texture_atlas_layout = texture_atlas_layout.add(layout);
        // let animation_indicies = AnimationIndicies { first: 1, last: 5 };
        // let texture = asset_server.load("fox-run.png");

        let texture = asset_server.load("cat/groom-1.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 21), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layout.add(layout);
        let animation_indicies = AnimationIndicies { first: 0, last: 3 };

        commands.spawn(Camera2dBundle::default());
        commands.spawn(CatBundle {
            state: CatState::IDLE,
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
