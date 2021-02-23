use asteroid::{
    component::{
        Acceleration, CollisionMask, Friction, PlayerControlled, Thrust, Velocity, OBSTACLE,
    },
    AsteroidPlugin,
};

use bevy::{
    app::App,
    asset::{AssetServer, Assets},
    ecs::{Commands, IntoSystem, Res, ResMut},
    math::Vec3,
    render::entity::Camera2dBundle,
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
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(ship_handle.clone().into()),
            transform: Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
            ..Default::default()
        })
        .with(Velocity::with_translation(0.0, 100.0))
        .with(Acceleration::default())
        .with(Thrust::default())
        .with(PlayerControlled)
        .with(Friction(1.0))
        .with(CollisionMask(OBSTACLE));
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(AsteroidPlugin)
        .add_startup_system(setup.system())
        .run();
}
