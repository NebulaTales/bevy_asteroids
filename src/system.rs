use crate::component::*;
use bevy::prelude::*;

pub fn floor_velocity_system(mut query: Query<Mut<Velocity>>) {
    for mut velocity in query.iter_mut() {
        if velocity.rotation.abs() <= 0.0001 {
            velocity.rotation = 0.0;
        }

        if velocity.translation.length_squared() <= 1.0 {
            velocity.translation.y = 0.0;
            velocity.translation.x = 0.0;
        }
    }
}

pub fn velocity_system(time: Res<Time>, mut query: Query<(Mut<Velocity>, Mut<Transform>)>) {
    let delta_time = f32::min(0.2, time.delta_seconds());

    for (velocity, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(velocity.rotation * delta_time));
        transform.translation.x += velocity.translation.x * delta_time;
        transform.translation.y += velocity.translation.y * delta_time;
    }
}

pub fn acceleration_system(
    time: Res<Time>,
    mut query: Query<(&Acceleration, &Transform, Mut<Velocity>)>,
) {
    let delta_time = f32::min(0.2, time.delta_seconds());

    for (acceleration, transform, mut velocity) in query.iter_mut() {
        velocity.rotation += acceleration.rotation * delta_time;

        let rotation = transform.rotation.to_axis_angle();
        let angle = std::f32::consts::PI / 2.0 + rotation.0.z * rotation.1;
        velocity.translation += Vec2::new(
            angle.cos() * acceleration.forward * delta_time,
            angle.sin() * acceleration.forward * delta_time,
        );
    }
}

pub fn friction_system(time: Res<Time>, mut query: Query<(&Friction, &mut Velocity)>) {
    let delta_time = f32::min(0.2, time.delta_seconds());
    for (friction, mut velocity) in query.iter_mut() {
        velocity.rotation *= 1.0 - bevy::math::clamp(2.0 * friction.0 * delta_time, 0.0, 1.0);
        velocity.translation *= 1.0 - bevy::math::clamp(friction.0 * delta_time, 0.0, 1.0);
    }
}

/// The thrust system adds creates the acceleration
pub fn thrust_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&Thrust, Mut<Acceleration>)>,
) {
    let forwards = keyboard.pressed(KeyCode::Up);
    let left = keyboard.pressed(KeyCode::Left);
    let right = keyboard.pressed(KeyCode::Right);

    for (thrust, mut acceleration) in query.iter_mut() {
        acceleration.rotation =
            if left { thrust.yaw } else { 0.0 } - if right { thrust.yaw } else { 0.0 };
        acceleration.forward = if forwards { thrust.forward } else { 0.0 }
    }
}
