use crate::{Velocity, Wrap};
use bevy::{
    app::{AppBuilder, Plugin},
    asset::Assets,
    core::{Time, Timer},
    ecs::system::{Commands, IntoSystem, Query, Res, ResMut},
    math::{Vec2, Vec3},
    render::color::Color,
    sprite::Sprite,
    sprite::{entity::SpriteBundle, ColorMaterial},
    transform::components::Transform,
};
use std::time::Duration;

pub struct FireSpawner {
    time_span: Option<Timer>,
}

impl Default for FireSpawner {
    fn default() -> Self {
        FireSpawner { time_span: None }
    }
}

fn spawn_single(
    commands: &mut Commands,
    materials: &mut Assets<ColorMaterial>,
    position: Vec3,
    velocity: Vec2,
) {
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_translation(position),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(Velocity::new(velocity, 0.0))
        .with(Wrap::from_count(2));
}

const INITIAL_SPEED: f32 = 400.0;
const FLOOR_SPEED: f32 = 200.0;

pub fn spawn_fire(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&mut FireSpawner, Option<&Transform>, Option<&Velocity>)>,
) {
    for (mut spawner, spawner_transform, spawner_velocity) in query.iter_mut() {
        let fire = if let Some(time_span) = &mut spawner.time_span {
            time_span.tick(time.delta()).just_finished()
        } else {
            spawner.time_span = Some(Timer::new(Duration::from_millis(250_u64), true));
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
            let angle = std::f32::consts::PI / 2.0 + rotation.0.z * rotation.1;

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
        app.add_system(spawn_fire.system());
    }
}
