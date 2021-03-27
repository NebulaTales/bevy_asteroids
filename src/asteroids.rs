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

#[derive(Copy, Clone, PartialEq)]
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

    let mut e = commands.spawn();
    e.insert_bundle(SpriteSheetBundle {
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
    .insert(asteroid);

    if asteroid != Asteroid::Tiny {
        e.insert(Wrap::default());
    }
}

#[derive(Copy, Clone)]
struct SpawnArea {
    center: Vec2,
    radius: Vec2,
}

impl SpawnArea {
    fn at(&self, angle: f32) -> Vec2 {
        Vec2::new(
            self.center.x + angle.cos() * self.radius.x,
            self.center.y + angle.sin() * self.radius.y,
        )
    }

    fn random_at(&self) -> Vec2 {
        self.at(thread_rng().gen_range(0.0..std::f32::consts::PI * 2.0))
    }

    fn random_to(&self, direction: &SpawnArea) -> (Vec2, Vec2) {
        let spawn_position = self.random_at();
        let direction_position = direction.random_at();
        let speed = thread_rng().gen_range(50_f32..150_f32);
        let direction = (direction_position - spawn_position).normalize() * speed;

        (spawn_position, direction)
    }
}

fn spawn_radius(
    number: u16,
    commands: &mut Commands,
    spawn_info: &SpawnerInfo,
    asteroid: Asteroid,
    position: SpawnArea,
    direction: SpawnArea,
) {
    let mut rng = thread_rng();

    for _ in 0..number {
        let (spawn_position, direction) = position.random_to(&direction);
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
    q_asteroids: Query<(Entity, &Asteroid, &Transform, Option<&Velocity>)>,
    q_collides_with: Query<(&Transform, Option<&Velocity>)>,
) {
    for collision in events.iter() {
        if let Ok((entity, asteroid, transform, velocity)) = q_asteroids.get(collision.source) {
            commands.entity(entity).despawn();

            if let Some(asteroid) = match asteroid {
                Asteroid::Big => Some(Asteroid::Small),
                Asteroid::Small => Some(Asteroid::Tiny),
                Asteroid::Tiny => None,
            } {
                let center = transform.translation.into();
                let position = SpawnArea {
                    center,
                    radius: Vec2::new(10.0, 10.0),
                };

                let source_velocity = if let Some(&velocity) = velocity {
                    velocity
                } else {
                    Default::default()
                };
                let (target_position, target_velocity) =
                    if let Ok((target_position, target_velocity)) =
                        q_collides_with.get(collision.target)
                    {
                        (
                            target_position.translation.into(),
                            if let Some(&target_velocity) = target_velocity {
                                target_velocity
                            } else {
                                Default::default()
                            },
                        )
                    } else {
                        (center, Default::default())
                    };

                let p = center + center - target_position
                    + source_velocity.translation * 2.0
                    + target_velocity.translation;

                let direction = SpawnArea {
                    center: p,
                    radius: Vec2::new(100.0, 100.0),
                };
                spawn_radius(3, &mut commands, &spawn_info, asteroid, position, direction);
            }
        }
    }
}

// TODO radius should be based on screen
fn key_spawner(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    spawn_info: Res<SpawnerInfo>,
) {
    if keyboard.just_pressed(KeyCode::S) {
        spawn_radius(
            1,
            &mut commands,
            &spawn_info,
            Asteroid::Big,
            SpawnArea {
                center: Default::default(),
                radius: Vec2::new(600.0, 600.0),
            },
            SpawnArea {
                center: Default::default(),
                radius: Vec2::new(100.0, 100.0),
            },
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
