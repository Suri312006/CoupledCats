use bevy::prelude::*;

use crate::game::state::CatState;

#[derive(Component, Default)]
pub struct Velocity(pub IVec2);

#[derive(Component, Default)]
pub struct Bounds(pub UVec2);

pub fn move_window(
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
