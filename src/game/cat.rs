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

#[derive(Resource, Clone)]
pub struct CatImageHandles {
    pub idle: Handle<Image>,
    pub lick: Handle<Image>,
    pub groom: Handle<Image>,
    pub walk: Handle<Image>,
    pub sleep: Handle<Image>,
    pub tap: Handle<Image>,
    pub jump: Handle<Image>,
    pub strech: Handle<Image>,
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

        let texture = asset_server.load("cat/groom.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 21), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layout.add(layout);
        let animation_indicies = AnimationIndicies { first: 0, last: 3 };

        let image_handles = CatImageHandles {
            idle: asset_server.load("cat/idle.png"),
            lick: asset_server.load("cat/lick.png"),
            groom: asset_server.load("cat/groom.png"),
            jump: asset_server.load("cat/jump.png"),
            walk: asset_server.load("cat/walk.png"),
            sleep: asset_server.load("cat/sleep.png"),
            tap: asset_server.load("cat/tap.png"),
            strech: asset_server.load("cat/strech.png"),
        };

        commands.insert_resource(image_handles);

        commands.spawn(Camera2dBundle::default());
        commands.spawn(CatBundle {
            state: CatState::STRECH,
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
