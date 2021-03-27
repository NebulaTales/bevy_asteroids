use crate::{CollisionLayer, CollisionMask};
use bevy::{
    app::{AppBuilder, Plugin},
    asset::{Assets, Handle},
    core::{Time, Timer},
    ecs::{
        entity::Entity,
        query::{With, Without},
        schedule::{ParallelSystemDescriptorCoercion, SystemLabel},
        system::{Commands, IntoSystem, Query, Res},
    },
    math::{Quat, Vec2, Vec3},
    render::camera::OrthographicProjection,
    sprite::{
        entity::{SpriteBundle, SpriteSheetBundle},
        ColorMaterial, Sprite, TextureAtlas, TextureAtlasSprite,
    },
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
pub fn spawn_ghosts_sprite(
    mut commands: Commands,
    q_projection: Query<&OrthographicProjection, With<WrapCamera>>,
    query: Query<
        (
            Entity,
            &Handle<ColorMaterial>,
            &Transform,
            &Sprite,
            Option<&CollisionMask>,
            Option<&CollisionLayer>,
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
                    let mut entity_commands = commands.spawn_bundle(SpriteBundle {
                        material: material.clone(),
                        transform: transform.clone(),
                        ..Default::default()
                    });

                    entity_commands.insert(Ghost::new(entity, *direction));

                    if let Some(collision_mask) = collision_mask {
                        entity_commands.insert(collision_mask.clone());
                    }
                    if let Some(layer_mask) = layer_mask {
                        entity_commands.insert(layer_mask.clone());
                    }
                    entities.push(Some(entity_commands.id()));
                }

                commands.entity(entity).insert(Wrapped {
                    ghosts: [entities[0], entities[1], entities[2]],
                });
            }
        }
    }
}

pub fn spawn_ghosts_sprite_atlas(
    mut commands: Commands,
    q_projection: Query<&OrthographicProjection, With<WrapCamera>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    query: Query<
        (
            Entity,
            &Handle<TextureAtlas>,
            &Transform,
            &TextureAtlasSprite,
            Option<&CollisionMask>,
            Option<&CollisionLayer>,
        ),
        (With<Wrap>, Without<Wrapped>),
    >,
) {
    for projection in q_projection.iter() {
        let screen_min = Vec2::new(projection.left, projection.bottom);
        let screen_max = Vec2::new(projection.right, projection.top);

        for (entity, texture_atlas, transform, sprite, collision_mask, layer_mask) in query.iter() {
            let (sprite_min, sprite_max) =
                if let Some(texture_atlas) = texture_atlases.get(texture_atlas) {
                    (
                        transform.translation.truncate() - texture_atlas.size / 2.0,
                        transform.translation.truncate() + texture_atlas.size / 2.0,
                    )
                } else {
                    (Default::default(), Default::default())
                };

            let sprite_in_screen = sprite_min.x > screen_min.x
                && sprite_min.y > screen_min.y
                && sprite_max.x < screen_max.x
                && sprite_max.y < screen_max.y;

            if sprite_in_screen {
                // TODO Should do better
                let mut entities = Vec::new();

                for direction in &[GDir::WestEast, GDir::NorthSouth, GDir::Diagonal] {
                    let mut entity_commands = commands.spawn_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas.clone(),
                        transform: transform.clone(),
                        sprite: TextureAtlasSprite {
                            index: sprite.index,
                            ..Default::default()
                        },
                        ..Default::default()
                    });

                    entity_commands.insert(Ghost::new(entity, *direction));

                    if let Some(collision_mask) = collision_mask {
                        entity_commands.insert(collision_mask.clone());
                    }
                    if let Some(layer_mask) = layer_mask {
                        entity_commands.insert(layer_mask.clone());
                    }
                    entities.push(Some(entity_commands.id()));
                }

                commands.entity(entity).insert(Wrapped {
                    ghosts: [entities[0], entities[1], entities[2]],
                });
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
            commands.entity(entity).despawn();
        }
    }
}

/// Direct despawner.
/// For any `Wrapped` entity whose `Wrap` tag has been removed, all ghosts
/// are removed, so is the `Wrapped` tag.
/// The `Wrap` tag must be removed manually to trigger this event.
/// This is only done if the ghost is not visible: if the main entity is in the screen
fn despawn_ghosts_direct_sprite(
    mut commands: Commands,
    q_projection: Query<&OrthographicProjection, With<WrapCamera>>,
    query: Query<(Entity, &Wrapped, &Transform, &Sprite), Without<Wrap>>,
) {
    for projection in q_projection.iter() {
        let screen_min = Vec2::new(projection.left, projection.bottom);
        let screen_max = Vec2::new(projection.right, projection.top);

        for (entity, wrapped, transform, sprite) in query.iter() {
            let sprite_min = transform.translation.truncate() - sprite.size / 2.0;
            let sprite_max = transform.translation.truncate() + sprite.size / 2.0;
            let sprite_in_screen = sprite_min.x > screen_min.x
                && sprite_min.y > screen_min.y
                && sprite_max.x < screen_max.x
                && sprite_max.y < screen_max.y;

            if sprite_in_screen {
                for ghost in wrapped.ghosts.iter() {
                    if let Some(ghost) = ghost {
                        commands.entity(*ghost).despawn();
                    }
                }
                commands.entity(entity).remove::<Wrapped>();
            }
        }
    }
}

fn despawn_ghosts_direct_sprite_atlas(
    mut commands: Commands,
    q_projection: Query<&OrthographicProjection, With<WrapCamera>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    query: Query<(Entity, &Wrapped, &Transform, &Handle<TextureAtlas>), Without<Wrap>>,
) {
    for projection in q_projection.iter() {
        let screen_min = Vec2::new(projection.left, projection.bottom);
        let screen_max = Vec2::new(projection.right, projection.top);

        for (entity, wrapped, transform, texture_atlas) in query.iter() {
            let (sprite_min, sprite_max) =
                if let Some(texture_atlas) = texture_atlases.get(texture_atlas) {
                    (
                        transform.translation.truncate() - texture_atlas.size / 2.0,
                        transform.translation.truncate() + texture_atlas.size / 2.0,
                    )
                } else {
                    (Default::default(), Default::default())
                };

            let sprite_in_screen = sprite_min.x > screen_min.x
                && sprite_min.y > screen_min.y
                && sprite_max.x < screen_max.x
                && sprite_max.y < screen_max.y;

            if sprite_in_screen {
                for ghost in wrapped.ghosts.iter() {
                    if let Some(ghost) = ghost {
                        commands.entity(*ghost).despawn();
                    }
                }
                commands.entity(entity).remove::<Wrapped>();
            }
        }
    }
}
/// Despawner for any main entity (None of `Ghost`, `Wrap` or `Wrapped`
/// When the sprite goes out of screen.
/// To see how an entity can lost its `Wrapped` tag, see `despawn_ghost`direct`
fn despawn_unwrapped_sprite(
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
            let out_of_right = position.x - sprite.size.x > projection.right;
            let out_of_left = position.x + sprite.size.x < projection.left;
            let out_of_top = position.y - sprite.size.y > projection.top;
            let out_of_bottom = position.y + sprite.size.y < projection.bottom;

            if out_of_right || out_of_left || out_of_top || out_of_bottom {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn despawn_unwrapped_sprite_atlas(
    mut commands: Commands,
    q_projection: Query<&OrthographicProjection, With<WrapCamera>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (Entity, &Handle<TextureAtlas>, &mut Transform),
        (Without<Wrap>, Without<Wrapped>, Without<Ghost>),
    >,
) {
    for projection in q_projection.iter() {
        for (entity, texture_atlas, mut transform) in query.iter_mut() {
            let size = if let Some(texture_atlas) = texture_atlases.get(texture_atlas) {
                texture_atlas.size
            } else {
                Default::default()
            };
            let position = &mut transform.translation;
            let out_of_right = position.x - size.x > projection.right;
            let out_of_left = position.x + size.x < projection.left;
            let out_of_top = position.y - size.y > projection.top;
            let out_of_bottom = position.y + size.y < projection.bottom;

            if out_of_right || out_of_left || out_of_top || out_of_bottom {
                commands.entity(entity).despawn();
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
        if matches!(wrap.remaining, Some(0))
            || if let Some(timer) = &mut wrap.timer {
                timer.tick(time.delta()).just_finished()
            } else {
                false
            }
        {
            commands.entity(entity).remove::<Wrap>();
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
                spawn_ghosts_sprite
                    .system()
                    .label(Label::Spawn)
                    .after(Label::Teleport),
            )
            .add_system(
                spawn_ghosts_sprite_atlas
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
            .add_system(despawn_ghosts_direct_sprite.system())
            .add_system(despawn_ghosts_direct_sprite_atlas.system())
            .add_system(auto_unwrap.system())
            .add_system(despawn_unwrapped_sprite.system())
            .add_system(despawn_unwrapped_sprite_atlas.system());
    }
}
