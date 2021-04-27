use crate::{
    Acceleration, Collider2D, CollisionEvent, CollisionLayer, CollisionMask, ControlLocked,
    Friction, PlayerControlled, PlayerLifes, Shape2D, Thrust, Velocity, Wrap, OBSTACLE, PLAYER,
};

use bevy::{
    app::{AppBuilder, EventReader, Plugin},
    asset::{AssetServer, Assets, Handle},
    core::{Time, Timer},
    ecs::{
        entity::Entity,
        query::{Added, With},
        system::{Commands, IntoSystem, Query, Res, ResMut},
    },
    math::Vec2,
    sprite::{entity::SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
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

fn destroy_on_collision(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut lifes: ResMut<PlayerLifes>,
    q_player: Query<Entity, With<Player>>,
) {
    let mut already_done = false;
    for collision in events.iter() {
        if let Ok(e) = q_player.get(collision.source) {
            assert!(
                !already_done,
                "This prooves the ship collided more than once!!!"
            );
            already_done = true;

            commands.entity(e).despawn();
            commands.spawn().insert(SpawnPlayer::default());

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
            animation: Timer::from_seconds(0.1, true),
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
) {
    commands.insert_resource(SpawnTexture(texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("sprites/ship.png"),
        Vec2::new(64.0, 64.0),
        13,
        1,
    ))));

    commands.spawn().insert(SpawnPlayer::default());
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
