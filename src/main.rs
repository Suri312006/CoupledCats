use bevy::window::WindowLevel;
use bevy::winit::winit_window_position;
use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    window::{Cursor, PrimaryWindow, WindowRef, WindowResolution},
    winit::WinitWindows,
};

#[derive(Resource, Deref, DerefMut)]
struct WindowMover {
    #[deref]
    pub speed: f32,
    pub position: f32,
}

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
        cursor: Cursor {
            hit_test: false,
            ..default()
        },
        ..default()
    };

    App::new()
        .insert_resource(ClearColor(Color::NONE))
        .insert_resource(WindowMover {
            speed: 20.0,
            position: 0.0,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(trans_window),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_window,))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    const CAT_X: f32 = 0.0;
    const CAT_Y: f32 = 0.0;
    const CAT_Z: f32 = 0.0;

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(CAT_X, CAT_Y, CAT_Z + 5.0),
        ..Default::default()
    });

    commands.spawn((
        Cat,
        SpriteBundle {
            texture: asset_server.load("neutral.png"),
            transform: Transform::from_xyz(CAT_X, CAT_Y, CAT_Z)
                .with_scale(Vec3::new(1.0, 1.0, 1.0)),
            visibility: Visibility::Visible,
            ..default()
        },
    ));
}

fn move_window(time: Res<Time>, mut windows: Query<&mut Window>, mut mover: ResMut<WindowMover>) {
    let mut window = windows.single_mut();

    mover.position += mover.speed * time.delta_seconds();

    let new_x = mover.position as i32;

    // window.position = WindowPosition::new(position);
    window.position.set(IVec2::new(new_x, 25.0 as i32));
}
