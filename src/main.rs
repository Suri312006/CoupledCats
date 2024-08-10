use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{WindowLevel, WindowResolution},
};

mod sprite_sheet;
use sprite_sheet::*;

#[derive(Component)]
struct Cat;

fn main() {
    let trans_window = Window {
        transparent: true,
        decorations: false,
        window_level: WindowLevel::AlwaysOnTop,
        resizable: false,
        resolution: WindowResolution::new(100.0, 100.0).with_scale_factor_override(1.0),
        position: WindowPosition::Centered(MonitorSelection::Current),
        ..default()
    };

    App::new()
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(trans_window),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), // needed for clear sprites,
        )
        // .add_plugins(LogDiagnosticsPlugin::default())
        // .add_plugins(FrameTimeDiagnosticsPlugin::default()) frame rate diagnostics
        .add_systems(Startup, setup)
        .add_systems(Update, (move_window, animate_sprite))
        .run();
}

fn move_window(time: Res<Time>, mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    let mut new_x = 0;
    let mut new_y = 0;

    match window.position {
        WindowPosition::At(ivec) => {
            new_x = ivec[0] + 1;
            new_y = ivec[1] + 0; // we not goin down
        }
        _ => {}
    }

    window.position.set(IVec2::new(new_x, new_y));
}
