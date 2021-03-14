use crate::{
    Acceleration, CollisionMask, DelayedAdd, Friction, LayerMask, PlayerControlled, Thrust,
    Velocity, Wrap, WrapCamera, OBSTACLE, PLAYER,
};

use bevy::{
    app::{AppBuilder, Plugin},
    asset::{AssetServer, Assets},
    core::Timer,
    ecs::system::{Commands, IntoSystem, Res, ResMut},
    math::Vec3,
    render::entity::OrthographicCameraBundle,
    sprite::{entity::SpriteBundle, ColorMaterial},
    transform::components::Transform,
};

pub fn player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ship_handle = asset_server.load("sprites/ship.png");

    commands
        .spawn(SpriteBundle {
            material: materials.add(ship_handle.into()),
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
        .with(CollisionMask(OBSTACLE))
        .with(Wrap::default())
        .with(LayerMask(PLAYER));
}

pub fn game(mut commands: Commands) {
    commands
        .spawn(OrthographicCameraBundle::new_2d())
        .with(WrapCamera);
}

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(player.system())
            .add_startup_system(game.system());
    }
}
