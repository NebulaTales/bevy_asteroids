use crate::{CollisionMask, LayerMask, Velocity, Wrap, OBSTACLE, PLAYER};
use std::time::Duration;

use bevy::{
    app::{AppBuilder, Plugin},
    asset::{AssetServer, Assets},
    core::{Time, Timer},
    ecs::system::{Commands, IntoSystem, Res, ResMut},
    math::{Vec2, Vec3},
    sprite::{entity::SpriteBundle, ColorMaterial},
    transform::components::Transform,
};

pub fn spawn_asteroids(
    number: u8,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    use rand::prelude::*;
    let texture_handle = asset_server.load("sprites/asteroids.png");
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

        let r = rng.gen_range(-10.0_f32..10.0_f32);

        commands
            .spawn(SpriteBundle {
                material: materials.add(texture_handle.clone().into()),
                transform: Transform::from_translation(Vec3::new(
                    spawn_position.x,
                    spawn_position.y,
                    0.0,
                )),
                ..Default::default()
            })
            .with(Velocity::new(Vec2::new(direction.x, direction.y), r))
            .with(LayerMask(OBSTACLE))
            .with(Wrap)
            .with(CollisionMask(PLAYER));
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
    materials: ResMut<Assets<ColorMaterial>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("SPAWN !");
        spawn_asteroids(5, commands, asset_server, materials);
    }
}

pub struct RulesPlugin;

impl Plugin for RulesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SpawnTimer>()
            .add_system(spawner.system());
    }
}
