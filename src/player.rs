use crate::{
    Acceleration, Collider2D, CollisionEvent, CollisionLayer, CollisionMask, FireAngleError,
    Friction, PlayerControlled, Shape2D, Thrust, Velocity, Wrap, OBSTACLE, PLAYER,
};

use bevy::{
    app::{AppBuilder, EventReader, Plugin},
    asset::{AssetServer, Assets, Handle},
    core::{Time, Timer},
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, IntoSystem, Query, Res, ResMut},
    },
    math::Vec2,
    sprite::{entity::SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
};

pub struct Player;
pub struct SpawnPlayer(Timer);
struct SpawnTexture(Handle<TextureAtlas>);

pub fn destroy_on_collision(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    q_player: Query<Entity, With<Player>>,
) {
    for collision in events.iter() {
        if let Ok(e) = q_player.get(collision.source) {
            commands.entity(e).despawn();
            commands.spawn().insert(SpawnPlayer::default());
        }
    }
}

impl Default for SpawnPlayer {
    fn default() -> Self {
        SpawnPlayer(Timer::from_seconds(3.0, false))
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
                    sprite: TextureAtlasSprite {
                        index: 0,
                        ..Default::default()
                    },
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
                .insert(CollisionMask(OBSTACLE))
                .insert(Wrap::default())
                .insert(Player)
                .insert(FireAngleError(0.03));
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
        2,
        1,
    ))));

    commands.spawn().insert(SpawnPlayer::default());
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system(spawn_player.system())
            .add_system(destroy_on_collision.system());
    }
}
