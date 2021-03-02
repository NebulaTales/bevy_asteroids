use crate::movement::{Acceleration, Thrust};
use bevy::{
    ecs::{Mut, Query, Res, With},
    input::{keyboard::KeyCode, Input},
};

#[derive(Copy, Clone)]
pub struct PlayerControlled;

/// The thrust system adds creates the acceleration using keyboard inputs
pub fn keyboard(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&Thrust, Mut<Acceleration>), With<PlayerControlled>>,
) {
    let up = keyboard.pressed(KeyCode::Up);
    let down = keyboard.pressed(KeyCode::Down);
    let left = keyboard.pressed(KeyCode::Left);
    let right = keyboard.pressed(KeyCode::Right);

    for (thrust, mut acceleration) in query.iter_mut() {
        acceleration.rotation =
            if left { thrust.yaw } else { 0.0 } - if right { thrust.yaw } else { 0.0 };
        acceleration.forward =
            if up { thrust.forward } else { 0.0 } - if down { thrust.backward } else { 0.0 };
    }
}
