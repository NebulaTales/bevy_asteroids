use crate::{
    AppState, AudioChannels, Collider2D, CollisionEvent, CollisionLayer, CollisionMask, Fire,
    NoWrapProtection, Score, Shape2D, SoundEffects, Velocity, Wrap, WrapCamera, AMMO, OBSTACLE,
    PLAYER, SCORE_BIG_ASTEROID, SCORE_SAUCER, SCORE_SMALL_ASTEROID, SCORE_TINY_ASTEROID,
};
use rand::prelude::*;
use std::{collections::HashSet, time::Duration};

use bevy::{
    app::{AppBuilder, EventReader, Plugin},
    asset::{AssetServer, Assets, Handle},
    core::{Time, Timer},
    ecs::{
        entity::Entity,
        query::{Or, With},
        schedule::SystemSet,
        system::{Commands, IntoSystem, Query, Res, ResMut},
    },
    math::Size,
    math::{Vec2, Vec3},
    render::{camera::OrthographicProjection, color::Color},
    sprite::{
        entity::{SpriteBundle, SpriteSheetBundle},
        ColorMaterial, Sprite, TextureAtlas, TextureAtlasSprite,
    },
    transform::components::Transform,
};
use bevy_kira_audio::Audio;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u16)]
enum Asteroid {
    Big = SCORE_BIG_ASTEROID,
    Small = SCORE_SMALL_ASTEROID,
    Tiny = SCORE_TINY_ASTEROID,
    Saucer = SCORE_SAUCER,
}

struct SpawnTexture(Handle<TextureAtlas>);
struct SpawnTimer(Timer, bool);
struct SaucerTimer(Timer);
struct ParticleColors(Vec<Handle<ColorMaterial>>);
struct SaucerParticleColors(Vec<Handle<ColorMaterial>>);

#[derive(Debug)]
struct Spawn {
    asteroid: Asteroid,
    position: Vec2,
    velocity: Vec2,
    spin: f32,
}

fn asteroid_scale(asteroid: Asteroid) -> f32 {
    match asteroid {
        Asteroid::Big => 1.0,
        Asteroid::Saucer => 0.75,
        Asteroid::Small => 0.5,
        Asteroid::Tiny => 0.25,
    }
}

fn spawn(
    mut commands: Commands,
    texture_atlas: Res<SpawnTexture>,
    q_spawn: Query<(Entity, &Spawn)>,
    audio: Res<Audio>,
    fx: Res<SoundEffects>,
    audio_channels: Res<AudioChannels>,
) {
    for (entity, spawn) in q_spawn.iter() {
        let mut rng = thread_rng();
        let scale = asteroid_scale(spawn.asteroid);

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
            .insert(spawn.asteroid)
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas.0.clone(),
                transform,
                sprite: TextureAtlasSprite {
                    index: if spawn.asteroid == Asteroid::Saucer {
                        0
                    } else {
                        rng.gen_range(1..5)
                    },
                    ..Default::default()
                },
                ..Default::default()
            });
        if spawn.asteroid != Asteroid::Tiny && spawn.asteroid != Asteroid::Saucer {
            e.insert(Wrap::default());
        }
        if spawn.asteroid == Asteroid::Saucer {
            e.insert(NoWrapProtection);
            audio.play_looped_in_channel(fx.ufo.clone(), &audio_channels.fx_ufo);
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
    if timer.1 && timer.0.tick(time.delta()).just_finished() {
        timer
            .0
            .set_duration(Duration::from_secs(rng.gen_range(1..5)));
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
    particle_colors: Res<ParticleColors>,
    saucer_particle_colors: Res<SaucerParticleColors>,
    q_asteroids: Query<(Entity, &Asteroid, &Transform, Option<&Velocity>)>,
    q_collides_with: Query<(&Transform, Option<&Velocity>)>,
    audio: Res<Audio>,
    fx: Res<SoundEffects>,
    audio_channels: Res<AudioChannels>,
) {
    // Ensuires each collision is treated once
    let mut already_done = HashSet::new();
    let mut rng = thread_rng();

    for collision in events.iter() {
        if let Ok((entity, asteroid, transform, velocity)) = q_asteroids.get(collision.source) {
            if already_done.contains(&entity) {
                continue;
            }
            commands.entity(entity).despawn();
            already_done.insert(entity);

            score.add(*asteroid as u16);

            let source_velocity = if let Some(&velocity) = velocity {
                velocity
            } else {
                Default::default()
            };

            if matches!(asteroid, Asteroid::Saucer) {
                audio.stop_channel(&audio_channels.fx_ufo);
            }
            audio.play_in_channel(fx.boom.clone(), &audio_channels.fx);

            if let Some(asteroid) = match asteroid {
                Asteroid::Big => Some(Asteroid::Small),
                Asteroid::Small => Some(Asteroid::Tiny),
                _ => None,
            } {
                let center = transform.translation.into();

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

                for _ in 0..rng.gen_range(2..5) {
                    commands.spawn().insert(SpawnRadius {
                        asteroid,
                        origin: (center, Size::new(10.0, 10.0)),
                        direction: (p, Size::new(100.0, 100.0)),
                    });
                }
            }

            // Generating particles

            let (count, radius, velocity_factor) = match asteroid {
                Asteroid::Saucer => (500.0, 32.0, 50.0),
                _ => {
                    let scale = asteroid_scale(*asteroid);
                    (200.0 * scale, 32.0 * scale, 1.0)
                }
            };

            for _ in 0..count as u16 {
                let size = {
                    let size = rng.gen_range(1.0..3.0);
                    Vec2::new(size, size)
                };

                let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
                let far = rng.gen_range(0.0..radius);

                let relative_position = Vec3::new(angle.cos() * far, angle.sin() * far, 0.0);

                let velocity =
                    source_velocity.translation + (relative_position * velocity_factor).into();

                let mut e = commands.spawn_bundle(SpriteBundle {
                    material: match asteroid {
                        Asteroid::Saucer => saucer_particle_colors.0
                            [rng.gen_range(0..saucer_particle_colors.0.len())]
                        .clone(),

                        _ => particle_colors.0[rng.gen_range(0..particle_colors.0.len())].clone(),
                    },
                    transform: Transform::from_translation(
                        transform.translation + relative_position,
                    ),
                    sprite: Sprite::new(size),
                    ..Default::default()
                });
                e.insert(Velocity::new(velocity, 0.0));

                if asteroid == &Asteroid::Saucer {
                    e.insert(Collider2D {
                        shape: Shape2D::Rectangle(size),
                        ..Default::default()
                    })
                    .insert(Fire)
                    .insert(CollisionLayer(AMMO))
                    .insert(CollisionMask(OBSTACLE));
                }
            }
        }
    }
}

pub struct AsteroidsPlugin;

fn prepare_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(SpawnTexture(texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("sprites/asteroids.png"),
        Vec2::new(64.0, 64.0),
        1,
        5,
    ))));

    commands.insert_resource(ParticleColors(vec![
        materials.add(Color::rgb(0.18, 0.18, 0.18).into()),
        materials.add(Color::rgb(0.23, 0.20, 0.20).into()),
        materials.add(Color::rgb(0.29, 0.26, 0.26).into()),
        materials.add(Color::rgb(0.36, 0.29, 0.29).into()),
        materials.add(Color::rgb(0.40, 0.32, 0.32).into()),
    ]));

    commands.insert_resource(SaucerParticleColors(vec![
        materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        materials.add(Color::rgb(1.0, 0.35, 0.0).into()),
        materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
    ]));
}

fn enter(mut commands: Commands, query: Query<Entity, With<Asteroid>>) {
    commands.insert_resource(SpawnTimer(Timer::from_seconds(1.0, true), true));
    commands.insert_resource(SaucerTimer(Timer::from_seconds(10.0, true)));

    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

fn exit(mut commands: Commands, query: Query<Entity, Or<(With<Spawn>, With<SpawnRadius>)>>) {
    commands.remove_resource::<SpawnTimer>();
    commands.remove_resource::<SaucerTimer>();

    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(prepare_resources.system())
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(enter.system()))
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(exit.system()))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(timed_spawn.system())
                    .with_system(saucer_timed_spawn.system())
                    .with_system(spawn.system())
                    .with_system(spawn_radius.system())
                    .with_system(destroy_on_collision.system()),
            );
    }
}
