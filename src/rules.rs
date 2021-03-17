use crate::{
    Collider2D, CollisionMask, LayerMask, Shape2D, Velocity, Wrap, AMMO, OBSTACLE, PLAYER,
};
use std::time::Duration;

use bevy::{
    app::{AppBuilder, Plugin},
    asset::{AssetServer, Assets},
    core::{Time, Timer},
    ecs::system::{Commands, IntoSystem, Res, ResMut},
    input::{keyboard::KeyCode, Input},
    math::{Vec2, Vec3},
    sprite::{entity::SpriteSheetBundle, TextureAtlas},
    transform::components::Transform,
};

fn spawn_single(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    position: Vec2,
    velocity: Vec2,
    spin: f32,
) {
    let texture_handle = asset_server.load("sprites/asteroids.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 1, 4);
    let texture_atlas = texture_atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas,
            transform: Transform::from_translation(Vec3::new(position.x, position.y, 10.0)),
            ..Default::default()
        })
        .with(Velocity::new(Vec2::new(velocity.x, velocity.y), spin))
        .with(Collider2D {
            shape: Shape2D::Circle(32.0),
            ..Default::default()
        })
        .with(LayerMask(OBSTACLE))
        .with(CollisionMask(PLAYER | AMMO))
        .with(Wrap::default());
}

pub fn spawn(
    number: u16,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    use rand::prelude::*;
    let mut rng = thread_rng();

    let spawn_radius = Vec2::new(600.0, 600.0);
    let direction_radius = Vec2::new(100.0, 100.0);

    for _ in 0..number {
        let spawn_angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
        let direction_angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);

        let spawn_position = Vec2::new(
            spawn_angle.cos() * spawn_radius.x,
            spawn_angle.sin() * spawn_radius.y,
        );
        let direction_position = Vec2::new(
            direction_angle.cos() * direction_radius.x,
            direction_angle.sin() * direction_radius.y,
        );
        let speed = rng.gen_range(50_f32..150_f32);
        let direction = (direction_position - spawn_position).normalize() * speed;

        let r = rng.gen_range(-5.0_f32..5.0_f32);

        spawn_single(
            &mut commands,
            &asset_server,
            &mut texture_atlases,
            spawn_position,
            direction,
            r,
        );
    }
}

struct SpawnTimer(Timer);

impl Default for SpawnTimer {
    fn default() -> Self {
        SpawnTimer(Timer::new(Duration::from_secs(3), false))
    }
}

fn spawner(
    commands: Commands,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        spawn(0, commands, asset_server, texture_atlases);
    }
}

fn key_spawner(
    commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if keyboard.just_pressed(KeyCode::S) {
        spawn(1, commands, asset_server, texture_atlases);
    }
}

pub struct RulesPlugin;

impl Plugin for RulesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SpawnTimer>()
            .add_system(spawner.system())
            .add_system(key_spawner.system());
    }
}
