use crate::{
    Collider2D, CollisionMask, LayerMask, Shape2D, Velocity, Wrap, AMMO, OBSTACLE, PLAYER,
};

use bevy::{
    app::{AppBuilder, Plugin},
    asset::{AssetServer, Assets, Handle},
    ecs::system::{Commands, IntoSystem, Res, ResMut},
    input::{keyboard::KeyCode, Input},
    math::{Vec2, Vec3},
    sprite::{entity::SpriteSheetBundle, TextureAtlas},
    transform::components::Transform,
};

fn spawn_single(
    commands: &mut Commands,
    spawn_info: &SpawnerInfo,
    position: Vec2,
    velocity: Vec2,
    spin: f32,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn_info.texture.clone(),
            transform: Transform::from_translation(Vec3::new(position.x, position.y, 10.0)),
            ..Default::default()
        })
        .insert(Velocity::new(Vec2::new(velocity.x, velocity.y), spin))
        .insert(Collider2D {
            shape: Shape2D::Circle(32.0),
            ..Default::default()
        })
        .insert(LayerMask(OBSTACLE))
        .insert(CollisionMask(PLAYER | AMMO))
        .insert(Wrap::default());
}

fn spawn(number: u16, mut commands: Commands, spawn_info: &SpawnerInfo) {
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

        spawn_single(&mut commands, &spawn_info, spawn_position, direction, r);
    }
}

fn key_spawner(commands: Commands, keyboard: Res<Input<KeyCode>>, spawn_info: Res<SpawnerInfo>) {
    if keyboard.just_pressed(KeyCode::S) {
        spawn(1, commands, &spawn_info);
    }
}

pub struct RulesPlugin;

struct SpawnerInfo {
    pub texture: Handle<TextureAtlas>,
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/asteroids.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 1, 4);

    commands.insert_resource(SpawnerInfo {
        texture: texture_atlases.add(texture_atlas),
    });
}

impl Plugin for RulesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system(key_spawner.system());
    }
}
