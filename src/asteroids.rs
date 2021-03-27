use crate::{
    Collider2D, CollisionEvent, CollisionLayer, CollisionMask, Shape2D, Velocity, Wrap, AMMO,
    OBSTACLE, PLAYER,
};
use rand::prelude::*;

use bevy::{
    app::{AppBuilder, EventReader, Plugin},
    asset::{AssetServer, Assets, Handle},
    ecs::{
        entity::Entity,
        system::{Commands, IntoSystem, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, Input},
    math::{Vec2, Vec3},
    sprite::{entity::SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    transform::components::Transform,
};

#[derive(Copy, Clone)]
enum Asteroid {
    Big,
    Small,
    Tiny,
}

fn spawn_single(
    commands: &mut Commands,
    spawn_info: &SpawnerInfo,
    asteroid: Asteroid,
    position: Vec2,
    velocity: Vec2,
    spin: f32,
) {
    let mut rng = thread_rng();
    let scale = match asteroid {
        Asteroid::Big => 1.0,
        Asteroid::Small => 0.5,
        Asteroid::Tiny => 0.25,
    };

    let transform = Transform::from_translation(Vec3::new(position.x, position.y, 10.0))
        * Transform::from_scale(Vec3::new(scale, scale, 1.0));

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn_info.texture.clone(),
            transform,
            sprite: TextureAtlasSprite {
                index: rng.gen_range(0..4),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Velocity::new(Vec2::new(velocity.x, velocity.y), spin))
        .insert(Collider2D {
            shape: Shape2D::Circle(32.0 * scale),
            ..Default::default()
        })
        .insert(CollisionLayer(OBSTACLE))
        .insert(CollisionMask(PLAYER | AMMO))
        .insert(asteroid)
        .insert(Wrap::default());
}

fn spawn_radius(
    number: u16,
    commands: &mut Commands,
    spawn_info: &SpawnerInfo,
    asteroid: Asteroid,
    center: Vec2,
    radius: Vec2,
    direction_radius: Vec2,
) {
    let mut rng = thread_rng();

    for _ in 0..number {
        let spawn_angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
        let direction_angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);

        let spawn_position = Vec2::new(
            center.x + spawn_angle.cos() * radius.x,
            center.y + spawn_angle.sin() * radius.y,
        );
        let direction_position = Vec2::new(
            center.x + direction_angle.cos() * direction_radius.x,
            center.y + direction_angle.sin() * direction_radius.y,
        );
        let speed = rng.gen_range(50_f32..150_f32);
        let direction = (direction_position - spawn_position).normalize() * speed;

        let r = rng.gen_range(-5.0_f32..5.0_f32);

        spawn_single(
            commands,
            &spawn_info,
            asteroid,
            spawn_position,
            direction,
            r,
        );
    }
}

// prints events as they come in
fn destroy_on_collision(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    spawn_info: Res<SpawnerInfo>,
    q_asteroids: Query<(Entity, &Asteroid, &Transform)>,
) {
    for collision in events.iter() {
        if let Ok((entity, asteroid, transform)) = q_asteroids.get(collision.source) {
            commands.entity(entity).despawn();

            if let Some(asteroid) = match asteroid {
                Asteroid::Big => Some(Asteroid::Small),
                Asteroid::Small => Some(Asteroid::Tiny),
                Asteroid::Tiny => None,
            } {
                spawn_radius(
                    3,
                    &mut commands,
                    &spawn_info,
                    asteroid,
                    transform.translation.into(),
                    Vec2::new(10.0, 10.0),
                    Vec2::new(15.0, 15.0),
                );
            }
            // For now spawn a new single one at the same place
        }
    }
}

fn key_spawner(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    spawn_info: Res<SpawnerInfo>,
) {
    let radius = Vec2::new(600.0, 600.0);
    let direction_radius = Vec2::new(100.0, 100.0);
    let center = Default::default();
    if keyboard.just_pressed(KeyCode::S) {
        spawn_radius(
            1,
            &mut commands,
            &spawn_info,
            Asteroid::Big,
            center,
            radius,
            direction_radius,
        );
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
            .add_system(key_spawner.system())
            .add_system(destroy_on_collision.system());
    }
}
