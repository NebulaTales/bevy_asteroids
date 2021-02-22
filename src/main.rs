use bevy::prelude::*;

#[derive(Default, Debug)]
struct Velocity(Vec3);

#[derive(Default, Debug)]
struct Acceleration(Vec3);

#[derive(Debug)]
struct Thrust {
    forward: f32,
    backward: f32,
}

impl Default for Thrust {
    fn default() -> Self {
        Thrust {
            forward: 100.0,
            backward: 50.0,
        }
    }
}

fn acceleration_system(time: Res<Time>, mut query: Query<(&Acceleration, &mut Velocity)>) {
    let delta_seconds = f32::min(0.2, time.delta_seconds());

    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.0 += acceleration.0 * delta_seconds;
    }
}

fn velocity_system(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    let delta_seconds = f32::min(0.2, time.delta_seconds());
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * delta_seconds;
        println!("{:?}", velocity);
    }
}

fn thrust_system(keyboard: Res<Input<KeyCode>>, mut query: Query<(&Thrust, &mut Acceleration)>) {
    let forwards = keyboard.pressed(KeyCode::Up);
    let backwards = keyboard.pressed(KeyCode::Down);
    for (thrust, mut acceleration) in query.iter_mut() {
        acceleration.0.y = if forwards { thrust.forward } else { 0.0 }
            - if backwards { thrust.backward } else { 0.0 };
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
        .with(Thrust::default());
}

struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ClearColor(Color::rgb(0.1, 0.0, 0.2)))
            .add_startup_system(setup.system())
            .add_system(velocity_system.system())
            .add_system(acceleration_system.system())
            .add_system(thrust_system.system());
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(AsteroidPlugin)
        .run();
}
