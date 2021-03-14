use crate::{CollisionMask, LayerMask};
use bevy::{
    app::{AppBuilder, Plugin},
    asset::Handle,
    core::{Time, Timer},
    ecs::{
        entity::Entity,
        query::{With, Without},
        schedule::{ParallelSystemDescriptorCoercion, SystemLabel},
        system::{Commands, IntoSystem, Query, Res},
    },
    math::{Quat, Vec2, Vec3},
    render::camera::OrthographicProjection,
    sprite::{entity::SpriteBundle, ColorMaterial, Sprite},
    transform::components::Transform,
};
use std::time::Duration;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum Label {
    Teleport,
    Spawn,
    Shift,
}

pub struct WrapCamera;

pub struct Wrap {
    remaining: Option<u8>,
    timer: Option<Timer>,
}

impl Default for Wrap {
    fn default() -> Self {
        Wrap {
            remaining: None,
            timer: None,
        }
    }
}

impl Wrap {
    pub fn from_count(remaining: u8) -> Self {
        Wrap {
            remaining: Some(remaining),
            timer: None,
        }
    }

    pub fn from_duration(duration: Duration) -> Self {
        Wrap {
            remaining: None,
            timer: Some(Timer::new(duration, false)),
        }
    }
}

pub struct Wrapped {
    pub ghosts: [Option<Entity>; 3],
}

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

/// Ghost creation function
/// This system looks for any valid entity with the `Wrap` tag and no `Wrapped` tag yet.
/// For each, it'll create 3 ghosts (tagged `Ghost`) that will position correctly using
/// `set_ghosts_shift` system.
/// The original entity also received the `Wrapped` tag.
pub fn spawn_ghosts(
    mut commands: Commands,
    q_projection: Query<&OrthographicProjection, With<WrapCamera>>,
    query: Query<
        (
            Entity,
            &Handle<ColorMaterial>,
            &Transform,
            &Sprite,
            Option<&CollisionMask>,
            Option<&LayerMask>,
        ),
        (With<Wrap>, Without<Wrapped>),
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
                // TODO Should do better
                let mut entities = Vec::new();

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
                    entities.push(commands.current_entity());
                }

                commands.insert(
                    entity,
                    Wrapped {
                        ghosts: [entities[0], entities[1], entities[2]],
                    },
                );
            }
        }
    }
}

/// Automatic despawner for any ghost whose target (originating entity) does
/// not exist anymore.
fn despawn_ghosts_indirect(
    mut commands: Commands,
    q_targets: Query<Entity, With<Wrapped>>,
    q_ghosts: Query<(Entity, &Ghost)>,
) {
    for (entity, ghost) in q_ghosts.iter() {
        if q_targets.get(ghost.target).is_err() {
            commands.despawn(entity);
        }
    }
}

/// Direct despawner.
/// For any `Wrapped` entity whose `Wrap` tag has been removed, all ghosts
/// are removed, so is the `Wrapped` tag.
/// The `Wrap` tag must be removed manually to trigger this event.
fn despawn_ghosts_direct(mut commands: Commands, query: Query<(Entity, &Wrapped), Without<Wrap>>) {
    for (entity, wrapped) in query.iter() {
        for ghost in wrapped.ghosts.iter() {
            if let Some(ghost) = ghost {
                commands.despawn(*ghost);
            }
        }
        commands.remove::<Wrapped>(entity);
    }
}

/// Despawner for any main entity (None of `Ghost`, `Wrap` or `Wrapped`
/// When the sprite goes out of screen.
/// To see how an entity can lost its `Wrapped` tag, see `despawn_ghost`direct`
fn despawn_unwrapped(
    mut commands: Commands,
    q_projection: Query<&OrthographicProjection, With<WrapCamera>>,
    mut query: Query<
        (Entity, &Sprite, &mut Transform),
        (Without<Wrap>, Without<Wrapped>, Without<Ghost>),
    >,
) {
    for projection in q_projection.iter() {
        for (entity, sprite, mut transform) in query.iter_mut() {
            let position = &mut transform.translation;
            if position.x - sprite.size.x > projection.right {
                commands.despawn(entity);
            }
            if position.x + sprite.size.x < projection.left {
                commands.despawn(entity);
            }
            if position.y - sprite.size.y > projection.top {
                commands.despawn(entity);
            }
            if position.y + sprite.size.y < projection.bottom {
                commands.despawn(entity);
            }
        }
    }
}

/// Teleporter for any non-`Ghost`, `Wrapped` entity.
/// It'll warp the entity to the other side of the screen as soon as it touches it.
fn teleport_wrapped(
    q_projection: Query<&OrthographicProjection, With<WrapCamera>>,
    mut query: Query<(&mut Transform, Option<&mut Wrap>), (Without<Ghost>, With<Wrapped>)>,
) {
    for projection in q_projection.iter() {
        let h_warp = projection.right - projection.left;
        let v_warp = projection.top - projection.bottom;

        for (mut transform, wrap) in query.iter_mut() {
            let position = &mut transform.translation;
            let mut dec_count = 0_u8;
            if position.x > projection.right {
                position.x -= h_warp;
                dec_count += 1;
            }
            if position.x < projection.left {
                position.x += h_warp;
                dec_count += 1;
            }
            if position.y > projection.top {
                position.y -= v_warp;
                dec_count += 1;
            }
            if position.y < projection.bottom {
                position.y += v_warp;
                dec_count += 1;
            }

            if let Some(mut wrap) = wrap {
                if let Some(c) = wrap.remaining {
                    wrap.remaining = Some(if c <= dec_count { 0 } else { c - dec_count });
                }
            }
        }
    }
}

fn auto_unwrap(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Wrap)>) {
    for (entity, mut wrap) in query.iter_mut() {
        if let Some(0) = wrap.remaining {
            println!("{:?} Dye dye dye!!!", entity);
            commands.remove::<Wrap>(entity);
        }

        if let Some(timer) = &mut wrap.timer {
            if timer.tick(time.delta()).just_finished() {
                println!("{:?} Dye dye dye!!!", entity);
                commands.remove::<Wrap>(entity);
            }
        }
    }
}

/// Recalculate the position of all ghosts according to their target.
/// It does not directly changes the transform, but configures a shift+rotation
/// information that is then used by `move_ghosts`
fn set_ghosts_shift(
    q_projection: Query<&OrthographicProjection, With<WrapCamera>>,
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

/// Replace the ghosts according to their calculated transformation
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
            .add_system(despawn_ghosts_indirect.system())
            .add_system(despawn_ghosts_direct.system())
            .add_system(auto_unwrap.system())
            .add_system(despawn_unwrapped.system());
    }
}
