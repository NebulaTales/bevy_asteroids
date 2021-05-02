use crate::{
    Acceleration, Collider2D, CollisionEvent, CollisionLayer, CollisionMask, ControlLocked, Fire,
    Friction, PlayerControlled, PlayerLifes, Shape2D, Thrust, Velocity, Wrap, AMMO, OBSTACLE,
    PLAYER,
};
use rand::prelude::*;
use std::collections::HashSet;

use bevy::{
    app::{AppBuilder, EventReader, Plugin},
    asset::{AssetServer, Assets, Handle},
    core::{Time, Timer},
    ecs::{
        entity::Entity,
        query::{Added, With},
        system::{Commands, IntoSystem, Query, Res, ResMut},
    },
    math::{Vec2, Vec3},
    render::color::Color,
    sprite::{
        entity::{SpriteBundle, SpriteSheetBundle},
        ColorMaterial, Sprite, TextureAtlas, TextureAtlasSprite,
    },
    transform::components::Transform,
};

struct Player;
struct SpawnPlayer(Timer);
struct SpawnTexture(Handle<TextureAtlas>);
struct Immunity {
    duration: Timer,
    animation: Timer,
}

const SPRITE_FULL_SHIELD: u32 = 11;
const SPRITE_NO_SHIELD: u32 = 12;
struct PlayerColors(Vec<Handle<ColorMaterial>>);

fn destroy_on_collision(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut lifes: ResMut<PlayerLifes>,
    colors: Res<PlayerColors>,
    q_player: Query<(Entity, &Velocity, &Transform), With<Player>>,
) {
    let mut rng = thread_rng();
    let mut already_done = HashSet::new();
    for collision in events.iter() {
        if let Ok((e, ship_velocity, ship_transform)) = q_player.get(collision.source) {
            if already_done.contains(&e) {
                continue;
            }
            already_done.insert(e);

            commands.entity(e).despawn();
            commands.spawn().insert(SpawnPlayer::default());

            // Create particles
            for _ in 0..200 as u16 {
                let size = {
                    let size = rng.gen_range(0.1..3.0);
                    Vec2::new(size, size)
                };

                let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
                let far = rng.gen_range(0.0..32.0);

                let relative_position = Vec3::new(angle.cos() * far, angle.sin() * far, 0.0);

                let velocity = ship_velocity.translation + relative_position.into();
                commands
                    .spawn_bundle(SpriteBundle {
                        material: colors.0[rng.gen_range(0..colors.0.len())].clone(),
                        transform: Transform::from_translation(
                            ship_transform.translation + relative_position,
                        ),
                        sprite: Sprite::new(size),
                        ..Default::default()
                    })
                    .insert(Velocity::new(velocity, 0.0))
                    .insert(Collider2D {
                        shape: Shape2D::Rectangle(size),
                        ..Default::default()
                    })
                    .insert(Fire)
                    .insert(CollisionLayer(AMMO))
                    .insert(CollisionMask(OBSTACLE));
            }

            lifes.0 -= 1;
        }
    }
}

impl Default for SpawnPlayer {
    fn default() -> Self {
        SpawnPlayer(Timer::from_seconds(3.0, false))
    }
}

impl Default for Immunity {
    fn default() -> Self {
        Immunity {
            duration: Timer::from_seconds(3.0, false),
            animation: Timer::from_seconds(0.06, true),
        }
    }
}

// Immunity system
// Any ship that with immunity checks for a timer
// When timer is finished, immunity is removed.
// Control is also given to the player
fn remove_immunity(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TextureAtlasSprite, &mut Immunity), With<Player>>,
) {
    for (id, mut sprite, mut immunity) in query.iter_mut() {
        if immunity.animation.tick(time.delta()).just_finished() {
            if sprite.index < SPRITE_FULL_SHIELD {
                sprite.index += 1;
            }
        }

        if immunity.duration.tick(time.delta()).just_finished() {
            sprite.index = SPRITE_NO_SHIELD;
            commands
                .entity(id)
                .remove::<Immunity>()
                .remove::<ControlLocked>()
                .insert(CollisionMask(OBSTACLE));
        }
    }
}

fn new_immunity(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TextureAtlasSprite), (With<Player>, Added<Immunity>)>,
) {
    for (e, mut sprite) in query.iter_mut() {
        if sprite.index > SPRITE_FULL_SHIELD {
            sprite.index = SPRITE_FULL_SHIELD;
        }

        commands
            .entity(e)
            .remove::<CollisionMask>()
            .insert(ControlLocked);
    }
}

fn spawn_player(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlas: Res<SpawnTexture>,
    mut q_spawn: Query<(Entity, &mut SpawnPlayer)>,
) {
    for (entity, mut spawn) in q_spawn.iter_mut() {
        if spawn.0.tick(time.delta()).just_finished() {
            commands
                .entity(entity)
                .remove::<SpawnPlayer>()
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas.0.clone(),
                    ..Default::default()
                })
                .insert(Velocity::default())
                .insert(Acceleration::default())
                .insert(Thrust::default())
                .insert(Friction(1.0))
                .insert(PlayerControlled)
                .insert(Collider2D {
                    shape: Shape2D::Circle(32.0),
                    ..Default::default()
                })
                .insert(CollisionLayer(PLAYER))
                .insert(Wrap::default())
                .insert(Player)
                .insert(ControlLocked)
                .insert(Immunity::default());
        }
    }
}

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(SpawnTexture(texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("sprites/ship.png"),
        Vec2::new(64.0, 64.0),
        13,
        1,
    ))));

    commands.spawn().insert(SpawnPlayer::default());

    commands.insert_resource(PlayerColors(vec![
        materials.add(Color::rgb(0.36, 0.43, 1.00).into()),
        materials.add(Color::rgb(0.37, 0.80, 0.89).into()),
        materials.add(Color::rgb(0.67, 0.20, 0.20).into()),
        materials.add(Color::rgb(0.27, 0.16, 0.16).into()),
        materials.add(Color::rgb(0.86, 0.90, 0.99).into()),
        materials.add(Color::rgb(0.47, 0.53, 0.55).into()),
    ]));
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system(spawn_player.system())
            .add_system(remove_immunity.system())
            .add_system(new_immunity.system())
            .add_system(destroy_on_collision.system());
    }
}
