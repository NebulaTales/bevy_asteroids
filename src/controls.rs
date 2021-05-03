use crate::{
    movement::{Acceleration, Thrust},
    AppState, Firing, Wrap,
};
use bevy::{
    app::{AppBuilder, Plugin},
    ecs::{
        entity::Entity,
        query::{With, Without},
        schedule::SystemSet,
        system::{Commands, IntoSystem, Query, Res},
    },
    input::{keyboard::KeyCode, Input},
};

#[derive(Copy, Clone)]
pub struct PlayerControlled;
pub struct ControlLocked;

/// The thrust system adds creates the acceleration using keyboard inputs
pub fn thrust_up_down(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<
        (&Thrust, &mut Acceleration),
        (With<PlayerControlled>, Without<ControlLocked>),
    >,
) {
    for (thrust, mut acceleration) in query.iter_mut() {
        acceleration.forward = if keyboard.pressed(KeyCode::Up) {
            thrust.forward
        } else {
            0.0
        } - if keyboard.pressed(KeyCode::Down) {
            thrust.backward
        } else {
            0.0
        }
    }
}

pub fn thrust_left_right(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&Thrust, &mut Acceleration), With<PlayerControlled>>,
) {
    let left = keyboard.pressed(KeyCode::Left);
    let right = keyboard.pressed(KeyCode::Right);

    for (thrust, mut acceleration) in query.iter_mut() {
        acceleration.rotation =
            if left { thrust.yaw } else { 0.0 } - if right { thrust.yaw } else { 0.0 };
    }
}

/// The thrust system adds creates the acceleration using keyboard inputs
pub fn fire(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<Entity, (With<PlayerControlled>, Without<ControlLocked>)>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for entity in query.iter_mut() {
            commands.entity(entity).insert(Firing);
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
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(thrust_up_down.system())
                .with_system(thrust_left_right.system())
                .with_system(fire.system())
                .with_system(debug.system()),
        );
    }
}
