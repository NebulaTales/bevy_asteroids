use crate::{
    Acceleration, Collider2D, CollisionLayer, CollisionMask, DelayedAdd, FireAngleError, Friction,
    PlayerControlled, Shape2D, Thrust, Velocity, Wrap, OBSTACLE, PLAYER,
};

use bevy::{
    app::{AppBuilder, Plugin},
    asset::{AssetServer, Assets},
    core::Timer,
    ecs::system::{Commands, IntoSystem, Res, ResMut},
    math::Vec3,
    sprite::{entity::SpriteBundle, ColorMaterial},
    transform::components::Transform,
};

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ship_handle = asset_server.load("sprites/ship.png");

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(ship_handle.into()),
            transform: Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
            ..Default::default()
        })
        .insert(Velocity::with_translation(0.0, 100.0))
        .insert(Acceleration::default())
        .insert(Thrust::default())
        .insert(Friction(1.0))
        .insert(DelayedAdd(
            PlayerControlled,
            Timer::from_seconds(2.0, false),
        ))
        .insert(Collider2D {
            shape: Shape2D::Circle(32.0),
            ..Default::default()
        })
        .insert(CollisionLayer(PLAYER))
        .insert(CollisionMask(OBSTACLE))
        .insert(Wrap::default())
        .insert(FireAngleError(0.03));
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system());
    }
}
