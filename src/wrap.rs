use crate::{CollisionMask, LayerMask};
use bevy::{
    app::{AppBuilder, Plugin},
    asset::Handle,
    ecs::{
        entity::Entity,
        query::With,
        schedule::{ParallelSystemDescriptorCoercion, SystemLabel},
        system::{Commands, IntoSystem, Query},
    },
    math::{Quat, Vec2, Vec3},
    render::camera::OrthographicProjection,
    sprite::{entity::SpriteBundle, ColorMaterial, Sprite},
    transform::components::Transform,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum Label {
    Teleport,
    Spawn,
    Shift,
}

pub struct Wrap;
pub struct Wrapped;

#[derive(Clone, Copy)]
enum GDir {
    WestEast = 0b01,
    NorthSouth = 0b10,
    Diagonal = 0b11,
}

pub struct Ghost {
    pub target: Entity,
    shift: Vec3,
    rotation: Quat,
    direction: GDir,
}

impl Ghost {
    fn new(target: Entity, direction: GDir) -> Self {
        Ghost {
            target,
            direction,
            shift: Default::default(),
            rotation: Default::default(),
        }
    }
}

pub fn spawn_ghosts(
    mut commands: Commands,
    q_projection: Query<&OrthographicProjection>,
    query: Query<
        (
            Entity,
            &Handle<ColorMaterial>,
            &Transform,
            &Sprite,
            Option<&CollisionMask>,
            Option<&LayerMask>,
        ),
        With<Wrap>,
    >,
) {
    for projection in q_projection.iter() {
        let screen_min = Vec2::new(projection.left, projection.bottom);
        let screen_max = Vec2::new(projection.right, projection.top);

        for (entity, material, transform, sprite, collision_mask, layer_mask) in query.iter() {
            let sprite_min = transform.translation.truncate() - sprite.size / 2.0;
            let sprite_max = transform.translation.truncate() + sprite.size / 2.0;

            let sprite_in_screen = sprite_min.x > screen_min.x
                && sprite_min.y > screen_min.y
                && sprite_max.x < screen_max.x
                && sprite_max.y < screen_max.y;

            if sprite_in_screen {
                for direction in &[GDir::WestEast, GDir::NorthSouth, GDir::Diagonal] {
                    commands
                        .spawn(SpriteBundle {
                            material: material.clone(),
                            transform: transform.clone(),
                            ..Default::default()
                        })
                        .with(Ghost::new(entity, *direction));
                    if let Some(collision_mask) = collision_mask {
                        commands.with(collision_mask.clone());
                    }
                    if let Some(layer_mask) = layer_mask {
                        commands.with(layer_mask.clone());
                    }
                }

                commands.remove::<Wrap>(entity).insert(entity, Wrapped);
            }
        }
    }
}

fn despawn_ghosts(
    mut commands: Commands,
    q_targets: Query<Entity>,
    q_ghosts: Query<(Entity, &Ghost)>,
) {
    for (entity, ghost) in q_ghosts.iter() {
        if q_targets.get(ghost.target).is_err() {
            commands.despawn(entity);
        }
    }
}

fn teleport_wrapped(
    q_projection: Query<&OrthographicProjection>,
    mut query: Query<&mut Transform, With<Wrapped>>,
) {
    for projection in q_projection.iter() {
        let h_warp = projection.right - projection.left;
        let v_warp = projection.top - projection.bottom;

        for mut transform in query.iter_mut() {
            let position = &mut transform.translation;
            if position.x > projection.right {
                position.x -= h_warp;
            }
            if position.x < projection.left {
                position.x += h_warp;
            }
            if position.y > projection.top {
                position.y -= v_warp;
            }
            if position.y < projection.bottom {
                position.y += v_warp;
            }
        }
    }
}

// For each Ghost, fetch the transform of its model and save its shift
fn set_ghosts_shift(
    q_projection: Query<&OrthographicProjection>,
    q_targets: Query<(Entity, &Transform)>,
    mut q_ghosts: Query<&mut Ghost>,
) {
    for projection in q_projection.iter() {
        let center = Vec2::new(
            (projection.top + projection.bottom) / 2.0,
            (projection.right + projection.left) / 2.0,
        );
        for mut ghost in q_ghosts.iter_mut() {
            if let Ok(&transform) = q_targets.get_component::<Transform>(ghost.target) {
                // First we need to determine the target relative position
                let relative_target_position = Vec2::new(
                    transform.translation.x - center.x,
                    transform.translation.y - center.y,
                );

                let ghost_direction = Vec3::new(
                    if relative_target_position.x > 0.0 {
                        projection.left
                    } else {
                        projection.right
                    } * ((ghost.direction as u8) & 0b01 > 0) as u8 as f32,
                    if relative_target_position.y < 0.0 {
                        projection.top
                    } else {
                        projection.bottom
                    } * ((ghost.direction as u8) & 0b10 > 0) as u8 as f32,
                    0.0,
                );

                let scale = 2.0;
                ghost.shift = transform.translation + ghost_direction * scale;

                ghost.rotation = transform.rotation;
            }
        }
    }
}

fn move_ghosts(mut query: Query<(&mut Transform, &Ghost)>) {
    for (mut transform, ghost) in query.iter_mut() {
        transform.translation = ghost.shift;
        transform.rotation = ghost.rotation;
    }
}

pub struct WrapPlugin;

impl Plugin for WrapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(teleport_wrapped.system().label(Label::Teleport))
            .add_system(
                spawn_ghosts
                    .system()
                    .label(Label::Spawn)
                    .after(Label::Teleport),
            )
            .add_system(
                set_ghosts_shift
                    .system()
                    .label(Label::Shift)
                    .after(Label::Spawn),
            )
            .add_system(move_ghosts.system().after(Label::Shift))
            .add_system(despawn_ghosts.system());
    }
}
