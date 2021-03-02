use bevy::{
    app::{AppBuilder, Plugin},
    core::Time,
    ecs::{IntoSystem, Mut, Query, Res},
    math::{Quat, Vec2},
    transform::components::Transform,
};

#[derive(Default, Debug)]
pub struct Velocity {
    pub translation: Vec2,
    pub rotation: f32,
}

impl Velocity {
    pub fn new(translation: Vec2, rotation: f32) -> Self {
        Velocity {
            translation,
            rotation,
        }
    }

    pub fn with_translation(x: f32, y: f32) -> Self {
        Velocity {
            translation: Vec2::new(x, y),
            ..Default::default()
        }
    }

    pub fn with_rotation(rotation: f32) -> Self {
        Velocity {
            rotation,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug)]
pub struct Friction(pub f32);

#[derive(Default, Debug)]
pub struct Acceleration {
    pub forward: f32,
    pub rotation: f32,
}

#[derive(Debug)]
pub struct Thrust {
    pub forward: f32,
    pub backward: f32,
    pub yaw: f32,
}

impl Default for Thrust {
    fn default() -> Self {
        Thrust {
            forward: 1000.0,
            backward: 300.0,
            yaw: 17.0,
        }
    }
}

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

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(acceleration.system())
            .add_system(floor_velocity.system())
            .add_system(velocity.system())
            .add_system(friction.system());
    }
}
