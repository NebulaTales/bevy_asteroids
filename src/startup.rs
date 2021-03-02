use crate::{
    Acceleration, CollisionMask, DelayedAdd, Friction, LayerMask, PlayerControlled, Thrust,
    Velocity, OBSTACLE, PLAYER,
};

use bevy::{
    asset::{AssetServer, Assets},
    core::Timer,
    ecs::{Commands, Res, ResMut},
    math::{Vec2, Vec3},
    render::{color::Color, entity::OrthographicCameraBundle},
    sprite::{entity::SpriteBundle, ColorMaterial, Sprite},
    transform::components::Transform,
};

pub fn asteroids(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    use rand::prelude::*;
    let texture_handle = asset_server.load("sprites/asteroids.png");
    for _ in 0..0 {
        let mut rng = thread_rng();
        let x = rng.gen_range(-300.0_f32..300_f32);
        let y = rng.gen_range(-300.0_f32..300_f32);
        let dx = rng.gen_range(-150.0_f32..150_f32);
        let dy = rng.gen_range(-150_f32..150_f32);
        let r = rng.gen_range(-10.0_f32..10.0_f32);

        commands
            .spawn(SpriteBundle {
                material: materials.add(texture_handle.clone().into()),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..Default::default()
            })
            .with(Velocity::new(Vec2::new(dx, dy), r))
            .with(LayerMask(OBSTACLE))
            .with(CollisionMask(PLAYER));
    }
}

pub fn environment(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    for ix in 0..16 {
        for iy in 0..16 {
            let x = (ix - 8) as f32 * 100.0;
            let y = (iy - 8) as f32 * 100.0;
            commands.spawn(SpriteBundle {
                material: materials.add(
                    Color::rgb(
                        0.0,
                        if ix % 2 == 0 { 0.0 } else { 0.5 },
                        if iy % 2 == 0 { 0.5 } else { 0.0 },
                    )
                    .into(),
                ),
                transform: Transform::from_xyz(x, y, 0.0),
                sprite: Sprite::new(Vec2::new(15.0, 15.0)),
                ..Default::default()
            });
        }
    }
}

pub fn player(
    commands: &mut Commands,
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
        .with(LayerMask(PLAYER));
}

//struct WrapCameraNorth(pub Entity);
//struct WrapCameraNorthWest(pub Entity);
//struct WrapCameraWest(pub Entity);
//struct WrapCameraSouthWest(pub Entity);
//struct WrapCameraSouth(pub Entity);
//struct WrapCameraSouthEast(pub Entity);
//struct WrapCameraEast(pub Entity);
//struct WrapCameraNorthEast(pub Entity);

pub fn game(commands: &mut Commands) {
    commands.spawn(OrthographicCameraBundle::new_2d());
}
