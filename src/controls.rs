use crate::{
    movement::{Acceleration, Thrust},
    utils,
};
use bevy::{
    app::{AppBuilder, Plugin},
    ecs::{
        query::With,
        system::{IntoSystem, Query, Res},
    },
    input::{keyboard::KeyCode, Input},
};

#[derive(Copy, Clone)]
pub struct PlayerControlled;

/// The thrust system adds creates the acceleration using keyboard inputs
pub fn keyboard(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&Thrust, &mut Acceleration), With<PlayerControlled>>,
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

pub struct ControlsPlugin;
impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(keyboard.system())
            .add_system(utils::delayed_add::<PlayerControlled>.system());
    }
}
