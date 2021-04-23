use crate::{
    Acceleration, Collider2D, CollisionEvent, CollisionLayer, CollisionMask, DelayedAdd,
    FireAngleError, Friction, PlayerControlled, Shape2D, Thrust, Velocity, Wrap, OBSTACLE, PLAYER,
};

use bevy::{
    app::{AppBuilder, EventReader, Plugin},
    asset::{AssetServer, Assets, Handle},
    core::Timer,
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, IntoSystem, Query, Res, ResMut},
    },
    math::Vec3,
    sprite::{entity::SpriteBundle, ColorMaterial},
    transform::components::Transform,
};

pub struct Player;

pub fn destroy_on_collision(
    _: Commands,
    mut events: EventReader<CollisionEvent>,
    q_player: Query<Entity, With<Player>>,
) {
    for collision in events.iter() {
        if let Ok(_) = q_player.get(collision.source) {
            println!("BOOM!");
        }
    }
}

pub struct SpawnPlayer;
pub struct PlayerMaterial(Handle<ColorMaterial>);

pub fn spawn_player(
    mut commands: Commands,
    player_material: Res<PlayerMaterial>,
    q_spawn: Query<Entity, With<SpawnPlayer>>,
) {
    for entity in q_spawn.iter() {
        commands
            .entity(entity)
            .remove::<SpawnPlayer>()
            .insert_bundle(SpriteBundle {
                material: player_material.0.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
                ..Default::default()
            })
            .insert(Velocity::with_translation(0.0, 100.0))
            .insert(Acceleration::default())
            .insert(Thrust::default())
            .insert(Friction(1.0))
            .insert(DelayedAdd(
                PlayerControlled,
                Timer::from_seconds(2.0, false),
            ))
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

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ship_handle = asset_server.load("sprites/ship.png");

    let player_material = materials.add(ship_handle.into());
    commands.insert_resource(PlayerMaterial(player_material.clone()));
    commands.spawn().insert(SpawnPlayer);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system(spawn_player.system())
            .add_system(destroy_on_collision.system());
    }
}
