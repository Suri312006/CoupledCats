use bevy::{
    // add these if you want diagnostics
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    utils::info,
    window::{Cursor, WindowLevel, WindowResolution},
};

use super::{
    animate::{animate_sprite, randomize_state},
    cat::{Bounds, Cat, StateQueue, Velocity},
    state::{update_state_from_queue, update_texture_from_state, CatState},
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
            // logging stuff
            // .add_plugins(LogDiagnosticsPlugin::default())
            // .add_plugins(FrameTimeDiagnosticsPlugin::default()) frame rate diagnostics
            .add_systems(Startup, Cat::setup)
            .add_systems(
                Update,
                (
                    update_state_from_queue,
                    move_window,
                    animate_sprite,
                    update_texture_from_state,
                    randomize_state,
                )
            )
            .run();
    }
}

// mode this into its own thing / work on this system
fn move_window(
    mut query: Query<(&mut Velocity, &Bounds, &mut Transform, &CatState)>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();

    let (mut vel, bounds, mut transform, state) = query.single_mut();

    let mut new_x = 0;
    let mut new_y = 0;

    if let WindowPosition::At(pos) = window.position {
        match state {
            CatState::JUMP => vel.0.x = 1,
            CatState::WALK => vel.0.x = 2,
            _ => vel.0.x = 0,
        };

        if pos.x >= (bounds.0.x as i32) {
            vel.0.x *= -1;
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        }
        if pos.x <= 0 {
            vel.0.x *= -1;
            transform.rotation = Quat::default();
        }

        new_x = pos.x + vel.0.x;
        new_y = pos.y + vel.0.y;
        trace!("x: {new_x:?}, y: {new_y:?}, x vel: {:#?}", vel.0.x);
    }

    window
        .size()
        .set(Box::new(Vec2::new(100.0, 100.0)))
        .map_err(|err| {
            println!("{err:#?}");
        })
        .unwrap();

    window.position.set(IVec2::new(new_x, new_y));
}
