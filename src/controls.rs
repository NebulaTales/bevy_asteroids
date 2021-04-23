use crate::{
    movement::{Acceleration, Thrust},
    Firing, Wrap,
};
use bevy::{
    app::{AppBuilder, Plugin},
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, IntoSystem, Query, Res},
    },
    input::{keyboard::KeyCode, Input},
};

#[derive(Copy, Clone)]
pub struct PlayerControlled;

/// The thrust system adds creates the acceleration using keyboard inputs
pub fn thrust(
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

/// The thrust system adds creates the acceleration using keyboard inputs
pub fn fire(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<Entity, With<PlayerControlled>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for entity in query.iter_mut() {
            commands.entity(entity).insert(Firing::default());
        }
    }

    if keyboard.just_released(KeyCode::Space) {
        for entity in query.iter_mut() {
            commands.entity(entity).remove::<Firing>();
        }
    }
}

pub fn debug(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    query: Query<Entity, With<Wrap>>,
) {
    let kill = keyboard.just_pressed(KeyCode::K);
    let unwrap = keyboard.just_pressed(KeyCode::U);

    for entity in query.iter() {
        if kill {
            commands.entity(entity).despawn();
        }
        if unwrap {
            commands.entity(entity).remove::<Wrap>();
        }
    }
}

pub struct ControlsPlugin;
impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(thrust.system())
            .add_system(fire.system())
            .add_system(debug.system());
    }
}
