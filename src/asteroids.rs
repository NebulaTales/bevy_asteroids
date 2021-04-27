use crate::{
    Collider2D, CollisionEvent, CollisionLayer, CollisionMask, NoWrapProtection, Score, Shape2D,
    Velocity, Wrap, WrapCamera, AMMO, OBSTACLE, PLAYER, SCORE_BIG_ASTEROID, SCORE_SAUCER,
    SCORE_SMALL_ASTEROID, SCORE_TINY_ASTEROID,
};
use rand::prelude::*;
use std::{collections::HashSet, time::Duration};

use bevy::{
    app::{AppBuilder, EventReader, Plugin},
    asset::{AssetServer, Assets, Handle},
    core::{Time, Timer},
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, IntoSystem, Query, Res, ResMut},
    },
    math::Size,
    math::{Vec2, Vec3},
    render::camera::OrthographicProjection,
    sprite::{
        entity::{SpriteBundle, SpriteSheetBundle},
        ColorMaterial, TextureAtlas, TextureAtlasSprite,
    },
    transform::components::Transform,
};

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u16)]
enum Asteroid {
    Big = SCORE_BIG_ASTEROID,
    Small = SCORE_SMALL_ASTEROID,
    Tiny = SCORE_TINY_ASTEROID,
    Saucer = SCORE_SAUCER,
}

struct SpawnTexture(Handle<TextureAtlas>);
struct SaucerTexture(Handle<ColorMaterial>);
struct SpawnTimer(Timer);
struct SaucerTimer(Timer);

#[derive(Debug)]
struct Spawn {
    asteroid: Asteroid,
    position: Vec2,
    velocity: Vec2,
    spin: f32,
}

fn spawn(
    mut commands: Commands,
    texture_atlas: Res<SpawnTexture>,
    saucer_texture: Res<SaucerTexture>,
    q_spawn: Query<(Entity, &Spawn)>,
) {
    for (entity, spawn) in q_spawn.iter() {
        let mut rng = thread_rng();
        let scale = match spawn.asteroid {
            Asteroid::Big => 1.0,
            Asteroid::Saucer => 0.75,
            Asteroid::Small => 0.5,
            Asteroid::Tiny => 0.25,
        };

        let transform =
            Transform::from_translation(Vec3::new(spawn.position.x, spawn.position.y, 10.0))
                * Transform::from_scale(Vec3::new(scale, scale, 1.0));

        let mut e = commands.entity(entity);
        e.remove::<Spawn>()
            .insert(Velocity::new(
                Vec2::new(spawn.velocity.x, spawn.velocity.y),
                spawn.spin,
            ))
            .insert(Collider2D {
                shape: Shape2D::Circle(32.0 * scale),
                ..Default::default()
            })
            .insert(CollisionLayer(OBSTACLE))
            .insert(CollisionMask(PLAYER | AMMO))
            .insert(spawn.asteroid);

        if spawn.asteroid == Asteroid::Saucer {
            e.insert_bundle(SpriteBundle {
                material: saucer_texture.0.clone(),
                transform,
                ..Default::default()
            })
            .insert(NoWrapProtection);
            println!("pouet");
        } else {
            e.insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas.0.clone(),
                transform,
                sprite: TextureAtlasSprite {
                    index: rng.gen_range(0..4),
                    ..Default::default()
                },
                ..Default::default()
            });
            if spawn.asteroid != Asteroid::Tiny {
                e.insert(Wrap::default());
            }
        }
    }
}

#[derive(Debug)]
struct SpawnRadius {
    asteroid: Asteroid,
    origin: (Vec2, Size<f32>),
    direction: (Vec2, Size<f32>),
}

fn spawn_radius(mut commands: Commands, q_spawn: Query<(Entity, &SpawnRadius)>) {
    let mut rng = thread_rng();
    for (entity, spawn) in q_spawn.iter() {
        let position = {
            let angle = thread_rng().gen_range(0.0..std::f32::consts::PI * 2.0);
            Vec2::new(
                spawn.origin.0.x + angle.cos() * spawn.origin.1.width,
                spawn.origin.0.y + angle.sin() * spawn.origin.1.height,
            )
        };

        let velocity = {
            let direction_position = {
                let angle = thread_rng().gen_range(0.0..std::f32::consts::PI * 2.0);
                Vec2::new(
                    spawn.direction.0.x + angle.cos() * spawn.direction.1.width,
                    spawn.direction.0.y + angle.sin() * spawn.direction.1.height,
                )
            };

            let speed = thread_rng().gen_range(50_f32..150_f32);
            (direction_position - position).normalize() * speed
        };

        commands
            .entity(entity)
            .remove::<SpawnRadius>()
            .insert(Spawn {
                position,
                velocity,
                asteroid: spawn.asteroid,
                spin: rng.gen_range(-5.0_f32..5.0_f32),
            });
    }
}

fn timed_spawn(
    mut commands: Commands,
    time: Res<Time>,
    q_projection: Query<&OrthographicProjection, With<WrapCamera>>,
    mut timer: ResMut<SpawnTimer>,
) {
    let mut rng = thread_rng();
    if timer.0.tick(time.delta()).just_finished() {
        timer
            .0
            .set_duration(Duration::from_secs(rng.gen_range(1..15)));
        if let Ok(projection) = q_projection.single() {
            let diameter = Size::new(
                projection.right - projection.left,
                projection.top - projection.bottom,
            );

            commands.spawn().insert(SpawnRadius {
                asteroid: Asteroid::Big,
                origin: (Default::default(), diameter),
                direction: (Default::default(), diameter / 2.0),
            });
        }
    }
}

fn saucer_timed_spawn(
    mut commands: Commands,
    time: Res<Time>,
    q_projection: Query<&OrthographicProjection, With<WrapCamera>>,
    mut timer: ResMut<SaucerTimer>,
) {
    let mut rng = thread_rng();
    if timer.0.tick(time.delta()).just_finished() {
        timer
            .0
            .set_duration(Duration::from_secs(15 + rng.gen_range(1..5)));
        if let Ok(projection) = q_projection.single() {
            let y = rng.gen_range(projection.bottom + 64.0..projection.top - 64.0);
            let (position, velocity, spin) = if rng.gen_bool(0.5) {
                (
                    Vec2::new(projection.left - 64.0, y),
                    Vec2::new(300.0, 0.0),
                    5.0,
                )
            } else {
                (
                    Vec2::new(projection.right + 64.0, y),
                    Vec2::new(-300.0, 0.0),
                    -5.0,
                )
            };

            commands.spawn().insert(Spawn {
                asteroid: Asteroid::Saucer,
                position,
                velocity,
                spin,
            });
        }
    }
}

/// On collision, an asteroid will despawn and, in place smaller asteroids will
/// spawn.
fn destroy_on_collision(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut score: ResMut<Score>,
    q_asteroids: Query<(Entity, &Asteroid, &Transform, Option<&Velocity>)>,
    q_collides_with: Query<(&Transform, Option<&Velocity>)>,
) {
    // Ensuires each collision is treated once
    let mut already_done = HashSet::new();

    for collision in events.iter() {
        if let Ok((entity, asteroid, transform, velocity)) = q_asteroids.get(collision.source) {
            if already_done.contains(&entity) {
                continue;
            }
            commands.entity(entity).despawn();
            already_done.insert(entity);

            score.add(*asteroid as u16);

            if let Some(asteroid) = match asteroid {
                Asteroid::Big => Some(Asteroid::Small),
                Asteroid::Small => Some(Asteroid::Tiny),
                _ => None,
            } {
                let center = transform.translation.into();

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

                for _ in 0..3 {
                    commands.spawn().insert(SpawnRadius {
                        asteroid,
                        origin: (center, Size::new(10.0, 10.0)),
                        direction: (p, Size::new(100.0, 100.0)),
                    });
                }
            }
        }
    }
}

pub struct AsteroidsPlugin;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(SpawnTexture(texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("sprites/asteroids.png"),
        Vec2::new(64.0, 64.0),
        1,
        4,
    ))));

    commands.insert_resource(SaucerTexture(
        materials.add(asset_server.load("sprites/saucer.png").into()),
    ));
    commands.insert_resource(SpawnTimer(Timer::from_seconds(1.0, true)));
    commands.insert_resource(SaucerTimer(Timer::from_seconds(10.0, true)));
}

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system(timed_spawn.system())
            .add_system(saucer_timed_spawn.system())
            .add_system(spawn.system())
            .add_system(spawn_radius.system())
            .add_system(destroy_on_collision.system());
    }
}
