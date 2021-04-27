use crate::{
    Collider2D, CollisionEvent, CollisionLayer, CollisionMask, Shape2D, Velocity, Wrap, AMMO,
    OBSTACLE,
};
use bevy::{
    app::{AppBuilder, EventReader, Plugin},
    asset::{Assets, Handle},
    core::{Time, Timer},
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, IntoSystem, Query, Res, ResMut},
    },
    math::Vec2,
    render::color::Color,
    sprite::Sprite,
    sprite::{entity::SpriteBundle, ColorMaterial},
    transform::components::Transform,
};
use rand::prelude::*;
use std::time::Duration;

pub struct Firing {
    time_span: Option<Timer>,
}

impl Default for Firing {
    fn default() -> Self {
        Firing { time_span: None }
    }
}

const FLOOR_SPEED: f32 = 200.0;
const INITIAL_SPEED: f32 = 500.0;
const PEW_PEW_SPEED: u64 = 300;
const PEW_PEW_SIZE: f32 = 3.0;
const FIRE_ANGLE_ERROR: f32 = 0.03;

struct Fire;
pub struct FireColors(Vec<Handle<ColorMaterial>>);

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

pub fn spawn_fires(
    mut commands: Commands,
    time: Res<Time>,
    colors: Res<FireColors>,
    mut query: Query<(&mut Firing, &Transform, Option<&Velocity>)>,
) {
    let mut rng = thread_rng();

    for (mut spawner, transform, spawner_velocity) in query.iter_mut() {
        let fire = if let Some(time_span) = &mut spawner.time_span {
            time_span.tick(time.delta()).just_finished()
        } else {
            spawner.time_span = Some(Timer::new(Duration::from_millis(PEW_PEW_SPEED), true));
            true
        };

        if fire {
            // Calculate initial velocity by computing vector*INITIAL_SPEED
            let rotation = transform.rotation.to_axis_angle();
            let mut angle = std::f32::consts::PI / 2.0 + rotation.0.z * rotation.1;

            let error = std::f32::consts::PI * FIRE_ANGLE_ERROR;
            angle += rng.gen_range(-error..error);

            let mut velocity = Vec2::new(angle.cos() * INITIAL_SPEED, angle.sin() * INITIAL_SPEED);
            if let Some(&spawner_velocity) = spawner_velocity {
                velocity += spawner_velocity.translation;
            }

            // If the final velocity norm is under a given floor, we re-set it
            if velocity.length() < FLOOR_SPEED {
                velocity = velocity.normalize() * FLOOR_SPEED;
            }

            let position = transform.translation;

            let size = Vec2::new(PEW_PEW_SIZE, PEW_PEW_SIZE);
            commands
                .spawn_bundle(SpriteBundle {
                    material: colors.0[rng.gen_range(0..colors.0.len())].clone(),
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
    }
}

fn startup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Added the palette of fire colors
    commands.insert_resource(FireColors(vec![
        materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        materials.add(Color::rgb(1.0, 0.35, 0.0).into()),
        materials.add(Color::rgb(1.0, 0.60, 0.0).into()),
        materials.add(Color::rgb(1.0, 0.81, 0.0).into()),
        materials.add(Color::rgb(1.0, 0.91, 0.03).into()),
    ]));
}

pub struct FirePlugin;

impl Plugin for FirePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system(spawn_fires.system())
            .add_system(destroy_on_collision.system());
    }
}
