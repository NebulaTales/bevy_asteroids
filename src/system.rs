use crate::component::*;
use bevy::{
    core::Time,
    ecs::{Entity, Mut, Query, Res},
    input::{keyboard::KeyCode, Input},
    math::{Quat, Vec2},
    sprite::{collide_aabb::collide, Sprite},
    transform::components::Transform,
};

pub fn floor_velocity(mut query: Query<Mut<Velocity>>) {
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

pub fn velocity(time: Res<Time>, mut query: Query<(Mut<Velocity>, Mut<Transform>)>) {
    let delta_time = f32::min(0.2, time.delta_seconds());

    for (velocity, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(velocity.rotation * delta_time));
        transform.translation.x += velocity.translation.x * delta_time;
        transform.translation.y += velocity.translation.y * delta_time;
    }
}

pub fn acceleration(time: Res<Time>, mut query: Query<(&Acceleration, &Transform, Mut<Velocity>)>) {
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

pub fn friction(time: Res<Time>, mut query: Query<(&Friction, &mut Velocity)>) {
    let delta_time = f32::min(0.2, time.delta_seconds());
    for (friction, mut velocity) in query.iter_mut() {
        velocity.rotation *= 1.0 - bevy::math::clamp(2.0 * friction.0 * delta_time, 0.0, 1.0);
        velocity.translation *= 1.0 - bevy::math::clamp(friction.0 * delta_time, 0.0, 1.0);
    }
}

/// The thrust system adds creates the acceleration
pub fn thrust(keyboard: Res<Input<KeyCode>>, mut query: Query<(&Thrust, Mut<Acceleration>)>) {
    let forwards = keyboard.pressed(KeyCode::Up);
    let left = keyboard.pressed(KeyCode::Left);
    let right = keyboard.pressed(KeyCode::Right);

    for (thrust, mut acceleration) in query.iter_mut() {
        acceleration.rotation =
            if left { thrust.yaw } else { 0.0 } - if right { thrust.yaw } else { 0.0 };
        acceleration.forward = if forwards { thrust.forward } else { 0.0 }
    }
}

pub fn collision(
    source_query: Query<(Entity, &Sprite, &Transform, &CollisionMask)>,
    target_query: Query<(Entity, &Sprite, &Transform, &LayerMask)>,
) {
    for (_, src_sprite, src_transform, src_mask) in source_query.iter() {
        for (_, tgt_sprite, tgt_transform, tgt_mask) in target_query.iter() {
            if collide(
                src_transform.translation,
                src_sprite.size,
                tgt_transform.translation,
                tgt_sprite.size,
            )
            .is_some()
            {
                if src_mask.0 & tgt_mask.0 > 0u8 {
                    println!("{:?}", src_transform);
                }
            }
        }
    }
}