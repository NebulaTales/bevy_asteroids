use bevy::prelude::*;

#[derive(Default, Debug)]
struct Velocity {
    translation: Vec2,
    rotation: f32,
}

#[derive(Default, Debug)]
struct Friction(f32);

#[derive(Default, Debug)]
struct Acceleration {
    forward: f32,
    rotation: f32,
}

#[derive(Debug)]
struct Thrust {
    forward: f32,
    yaw: f32,
}

impl Default for Thrust {
    fn default() -> Self {
        Thrust {
            forward: 1000.0,
            yaw: 17.0,
        }
    }
}

fn floor_velocity_system(mut query: Query<Mut<Velocity>>) {
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

fn velocity_system(time: Res<Time>, mut query: Query<(Mut<Velocity>, Mut<Transform>)>) {
    let delta_time = f32::min(0.2, time.delta_seconds());

    for (velocity, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(velocity.rotation * delta_time));
        transform.translation.x += velocity.translation.x * delta_time;
        transform.translation.y += velocity.translation.y * delta_time;
    }
}

fn acceleration_system(
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

fn friction_system(time: Res<Time>, mut query: Query<(&Friction, &mut Velocity)>) {
    let delta_time = f32::min(0.2, time.delta_seconds());
    for (friction, mut velocity) in query.iter_mut() {
        velocity.rotation *= 1.0 - bevy::math::clamp(2.0 * friction.0 * delta_time, 0.0, 1.0);
        velocity.translation *= 1.0 - bevy::math::clamp(friction.0 * delta_time, 0.0, 1.0);
    }
}

/// The thrust system adds creates the acceleration
fn thrust_system(keyboard: Res<Input<KeyCode>>, mut query: Query<(&Thrust, Mut<Acceleration>)>) {
    let forwards = keyboard.pressed(KeyCode::Up);
    let left = keyboard.pressed(KeyCode::Left);
    let right = keyboard.pressed(KeyCode::Right);

    for (thrust, mut acceleration) in query.iter_mut() {
        acceleration.rotation =
            if left { thrust.yaw } else { 0.0 } - if right { thrust.yaw } else { 0.0 };
        acceleration.forward = if forwards { thrust.forward } else { 0.0 }
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ship_handle = asset_server.load("sprites/ship.png");
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(ship_handle.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(Velocity::default())
        .with(Acceleration::default())
        .with(Thrust::default())
        .with(Friction(1.0));
}

struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ClearColor(Color::rgb(0.1, 0.0, 0.2)))
            .add_startup_system(setup.system())
            .add_system(acceleration_system.system())
            .add_system(thrust_system.system())
            .add_system(floor_velocity_system.system())
            .add_system(velocity_system.system())
            .add_system(friction_system.system());
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(AsteroidPlugin)
        .run();
}
