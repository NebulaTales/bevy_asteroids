use crate::{
    Collider2D, CollisionEvent, CollisionLayer, CollisionMask, Shape2D, Velocity, Wrap, AMMO,
    OBSTACLE,
};
use bevy::{
    app::{AppBuilder, EventReader, Plugin},
    asset::Assets,
    core::{Time, Timer},
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, IntoSystem, Query, Res, ResMut},
    },
    math::{Vec2, Vec3},
    render::color::Color,
    sprite::Sprite,
    sprite::{entity::SpriteBundle, ColorMaterial},
    transform::components::Transform,
};
use std::time::Duration;

pub struct Firing {
    time_span: Option<Timer>,
}

pub struct FireAngleError(pub f32);

impl Default for Firing {
    fn default() -> Self {
        Firing { time_span: None }
    }
}

const BULLET_SIZE: f32 = 3.0;
const FLOOR_SPEED: f32 = 200.0;
const INITIAL_SPEED: f32 = 400.0;
const PEW_PEW_SPEED: u64 = 1;

struct Fire;

fn spawn_single(
    commands: &mut Commands,
    materials: &mut Assets<ColorMaterial>,
    position: Vec3,
    velocity: Vec2,
) {
    let size = Vec2::new(BULLET_SIZE, BULLET_SIZE);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_translation(position),
            sprite: Sprite::new(size),
            ..Default::default()
        })
        .insert(Velocity::new(velocity, 0.0))
        .insert(Wrap::from_count(1))
        .insert(Collider2D {
            shape: Shape2D::Rectangle(size),
            ..Default::default()
        })
        .insert(Fire)
        .insert(CollisionLayer(AMMO))
        .insert(CollisionMask(OBSTACLE));
}

fn destroy_on_collision(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    query: Query<Entity, With<Fire>>,
) {
    for collision in events.iter() {
        if let Ok(id) = query.get(collision.source) {
            commands.entity(id).despawn();
        }
    }
}

pub fn spawn_fire(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(
        &mut Firing,
        Option<&Transform>,
        Option<&Velocity>,
        Option<&FireAngleError>,
    )>,
) {
    use rand::prelude::*;
    let mut rng = thread_rng();

    for (mut spawner, spawner_transform, spawner_velocity, angle_error) in query.iter_mut() {
        let fire = if let Some(time_span) = &mut spawner.time_span {
            time_span.tick(time.delta()).just_finished()
        } else {
            spawner.time_span = Some(Timer::new(Duration::from_millis(PEW_PEW_SPEED), true));
            true
        };

        if fire {
            let transform = if let Some(&transform) = spawner_transform {
                transform
            } else {
                Default::default()
            };

            // Calculate initial velocity by computing vector*INITIAL_SPEED
            let rotation = transform.rotation.to_axis_angle();
            let mut angle = std::f32::consts::PI / 2.0 + rotation.0.z * rotation.1;

            if let Some(error) = angle_error {
                let error = std::f32::consts::PI * error.0;
                angle += rng.gen_range(-error..error);
            }

            let mut velocity = Vec2::new(angle.cos() * INITIAL_SPEED, angle.sin() * INITIAL_SPEED);
            if let Some(&spawner_velocity) = spawner_velocity {
                velocity += spawner_velocity.translation;
            }

            // If the final velocity norm is under a given floor, we re-set it
            if velocity.length() < FLOOR_SPEED {
                velocity = velocity.normalize() * FLOOR_SPEED;
            }

            let position = transform.translation;

            spawn_single(&mut commands, &mut materials, position, velocity);
        }
    }
}

pub struct FirePlugin;

impl Plugin for FirePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(spawn_fire.system())
            .add_system(destroy_on_collision.system());
    }
}
