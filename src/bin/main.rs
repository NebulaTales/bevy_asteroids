use asteroid::{
    component::{
        Acceleration, CollisionMask, DelayedAdd, Friction, PlayerControlled, Thrust, Velocity,
        OBSTACLE,
    },
    AsteroidPlugin,
};

use bevy::{
    app::App,
    asset::{AssetServer, Assets},
    core::Timer,
    ecs::{Commands, IntoSystem, Res, ResMut},
    math::Vec3,
    render::{color::Color, entity::OrthographicCameraBundle, pass::ClearColor},
    sprite::{entity::SpriteBundle, ColorMaterial},
    transform::components::Transform,
    DefaultPlugins,
};

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ship_handle = asset_server.load("sprites/ship.png");
    commands
        .spawn(OrthographicCameraBundle::new_2d())
        .spawn(SpriteBundle {
            material: materials.add(ship_handle.clone().into()),
            transform: Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
            ..Default::default()
        })
        .with(Velocity::with_translation(0.0, 100.0))
        .with(Acceleration::default())
        .with(Thrust::default())
        .with(Friction(1.0))
        .with(DelayedAdd(
            PlayerControlled,
            Timer::from_seconds(2.0, false),
        ))
        .with(CollisionMask(OBSTACLE));
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.2)))
        .add_plugins(DefaultPlugins)
        .add_plugin(AsteroidPlugin)
        .add_startup_system(setup.system())
        .run();
}
