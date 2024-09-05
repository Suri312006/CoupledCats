mod animate;
mod app;
mod movement;
mod state;
mod texture;

use std::collections::VecDeque;

pub use app::*;

use bevy::prelude::*;

use movement::*;
use state::*;
use animate::*;

#[derive(Component)]
pub struct Cat {}

#[derive(Bundle)]
struct CatBundle {
    state_queue: StateQueue<CatState>,
    state: CatState,
    velocity: Velocity,
    bounds: Bounds,
    sprite: SpriteBundle,
    texture_atlas: TextureAtlas,
    animation_timer: AnimationTimer,
    animation_indicies: AnimationIndicies,
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

// cat has sprite stuff
impl Cat {
    pub fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        //TODO: bind it to one monitor

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
            state_queue: StateQueue(VecDeque::new()),
            state: CatState::JUMP,
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
