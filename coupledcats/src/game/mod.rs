use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    window::{CursorOptions, WindowLevel, WindowResolution},
};
use systems::init_systems;

mod cat;
mod systems;

pub struct CoupledCats {}

impl CoupledCats {
    pub fn run(mut app: App) {
        let trans_window = Window {
            transparent: true,
            decorations: false,
            window_level: WindowLevel::AlwaysOnTop,
            resizable: false,
            resolution: WindowResolution::new(200.0, 200.0).with_scale_factor_override(1.0),
            position: WindowPosition::Centered(MonitorSelection::Current),
            cursor_options: CursorOptions {
                hit_test: false,
                ..default()
            },
            ..default()
        };

        app.insert_resource(ClearColor(Color::NONE)).add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(trans_window),
                    ..default()
                })
                .set(LogPlugin {
                    level: Level::DEBUG,
                    filter:
                        "wgpu_core=warn,wgpu_hal=warn,h2=warn,naga=warn,bevy_time=warn,tower=warn"
                            .into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), // needed for clear sprites,
        );

        init_systems(&mut app);

        app.run();
    }
}
