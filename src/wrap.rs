use crate::{AppState, CollisionLayer, CollisionMask};
use bevy::{
    app::{AppBuilder, Plugin},
    asset::{Assets, Handle},
    core::{Time, Timer},
    ecs::{
        entity::Entity,
        query::{With, Without},
        schedule::{ParallelSystemDescriptorCoercion, SystemLabel, SystemSet},
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
    Make,
}

pub struct WrapCamera;
pub struct NoWrapProtection;

pub struct Wrap {
    remaining: Option<u8>,
    timer: Option<Timer>,
    first_pass: bool,
}

impl Default for Wrap {
    fn default() -> Self {
        Wrap {
            remaining: None,
            timer: None,
            first_pass: true,
        }
    }
}

impl Wrap {
    pub fn from_count(remaining: u8) -> Self {
        Wrap {
            remaining: Some(remaining),
            timer: None,
            ..Default::default()
        }
    }

    pub fn from_duration(duration: Duration) -> Self {
        Wrap {
            remaining: None,
            timer: Some(Timer::new(duration, false)),
            ..Default::default()
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
    index: Option<u32>,
}

impl Ghost {
    fn new(target: Entity, direction: GDir) -> Self {
        Ghost {
            target,
            direction,
            shift: Default::default(),
            rotation: Default::default(),
            index: None,
        }
    }
}

pub struct Area {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Area {
    fn new(position: Vec2, size: Vec2) -> Area {
        let min = position - size / 2.0;
        let max = position + size / 2.0;

        Area {
            left: min.x,
            right: max.x,
            top: max.y,
            bottom: min.y,
        }
    }

    fn from_position_atlas(position: Vec2, texture_atlas: &TextureAtlas, index: usize) -> Area {
        let texture_rect = texture_atlas.textures[index as usize];
        let size = Vec2::new(texture_rect.width(), texture_rect.height());
        Area::new(position, size)
    }

    fn from_projection(projection: &OrthographicProjection) -> Area {
        Area {
            left: projection.left,
            right: projection.right,
            top: projection.top,
            bottom: projection.bottom,
        }
    }

    fn inside(&self, rect: &Self) -> bool {
        rect.right > self.right
            && rect.left < self.left
            && rect.top > self.top
            && rect.bottom < self.bottom
    }

    fn outside(&self, rect: &Self) -> bool {
        self.bottom > rect.top
            || self.top < rect.bottom
            || self.right < rect.left
            || self.left > rect.right
    }

    fn overlap(&self, rect: &Self) -> bool {
        !self.outside(rect)
    }

    fn center(&self) -> Vec2 {
        Vec2::new(
            (self.left + self.right) / 2.0,
            (self.bottom + self.top) / 2.0,
        )
    }

    fn size(&self) -> Vec2 {
        Vec2::new(self.right - self.left, self.top - self.bottom)
    }

    fn distance_squared(&self, point: Vec2) -> f32 {
        let size = self.size();
        let center = self.center();
        let dx = ((point.x - center.x).abs() - size.x / 2.0).max(0.0);
        let dy = ((point.y - center.y).abs() - size.y / 2.0).max(0.0);
        return dx * dx + dy * dy;
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
    mut query: Query<
        (
            Entity,
            &mut Wrap,
            &Handle<ColorMaterial>,
            &Transform,
            &Sprite,
            Option<&CollisionMask>,
            Option<&CollisionLayer>,
        ),
        Without<Wrapped>,
    >,
) {
    if let Ok(projection) = q_projection.single() {
        let screen_rect = Area::from_projection(&projection);

        for (entity, mut wrap, material, transform, sprite, collision_mask, layer_mask) in
            query.iter_mut()
        {
            let sprite_rect = Area::new(transform.translation.truncate(), sprite.size);

            let check = (wrap.first_pass && sprite_rect.overlap(&screen_rect))
                || sprite_rect.inside(&screen_rect);

            if check {
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

            wrap.first_pass = false;
        }
    }
}

pub fn spawn_ghosts_sprite_atlas(
    mut commands: Commands,
    q_projection: Query<&OrthographicProjection, With<WrapCamera>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            Entity,
            &mut Wrap,
            &Handle<TextureAtlas>,
            &Transform,
            &TextureAtlasSprite,
            Option<&CollisionMask>,
            Option<&CollisionLayer>,
        ),
        Without<Wrapped>,
    >,
) {
    if let Ok(projection) = q_projection.single() {
        let screen_rect = Area::from_projection(&projection);

        for (
            entity,
            mut wrap,
            texture_atlas_handle,
            transform,
            sprite,
            collision_mask,
            layer_mask,
        ) in query.iter_mut()
        {
            if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
                let sprite_rect = Area::from_position_atlas(
                    transform.translation.truncate(),
                    texture_atlas,
                    sprite.index as usize,
                );

                let check = (wrap.first_pass && sprite_rect.overlap(&screen_rect))
                    || sprite_rect.inside(&screen_rect);

                if check {
                    let mut entities = Vec::new();

                    for direction in &[GDir::WestEast, GDir::NorthSouth, GDir::Diagonal] {
                        let mut entity_commands = commands.spawn_bundle(SpriteSheetBundle {
                            texture_atlas: texture_atlas_handle.clone(),
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

                wrap.first_pass = false;
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
    if let Ok(projection) = q_projection.single() {
        let screen_rect = Area::from_projection(projection);

        for (entity, wrapped, transform, sprite) in query.iter() {
            let sprite_rect = Area::new(transform.translation.truncate(), sprite.size);

            if sprite_rect.inside(&screen_rect) {
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
    query: Query<
        (
            Entity,
            &Wrapped,
            &Transform,
            &Handle<TextureAtlas>,
            &TextureAtlasSprite,
        ),
        Without<Wrap>,
    >,
) {
    if let Ok(projection) = q_projection.single() {
        let screen_rect = Area::from_projection(projection);

        for (entity, wrapped, transform, texture_atlas, sprite) in query.iter() {
            if let Some(texture_atlas) = texture_atlases.get(texture_atlas) {
                let sprite_rect = Area::from_position_atlas(
                    transform.translation.truncate(),
                    texture_atlas,
                    sprite.index as usize,
                );

                if sprite_rect.inside(&screen_rect) {
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
}

/// Despawner for any main entity (None of `Ghost`, `Wrap` or `Wrapped`
/// When the sprite goes out of screen.
/// To see how an entity can lost its `Wrapped` tag, see `despawn_ghost`direct`
/// Added a marker that protects the entity from despawn, just in case.
fn despawn_unwrapped_sprite(
    mut commands: Commands,
    q_projection: Query<&OrthographicProjection, With<WrapCamera>>,
    mut query: Query<
        (Entity, &Sprite, &mut Transform),
        (
            Without<Wrap>,
            Without<Wrapped>,
            Without<Ghost>,
            Without<NoWrapProtection>,
        ),
    >,
) {
    if let Ok(projection) = q_projection.single() {
        let screen_rect = Area::from_projection(projection);
        for (entity, sprite, transform) in query.iter_mut() {
            let sprite_rect = Area::new(transform.translation.truncate(), sprite.size);

            if sprite_rect.outside(&screen_rect) {
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
        (
            Entity,
            &Handle<TextureAtlas>,
            &mut Transform,
            &TextureAtlasSprite,
        ),
        (
            Without<Wrap>,
            Without<Wrapped>,
            Without<Ghost>,
            Without<NoWrapProtection>,
        ),
    >,
) {
    if let Ok(projection) = q_projection.single() {
        let screen_rect = Area::from_projection(projection);
        for (entity, texture_atlas, transform, sprite) in query.iter_mut() {
            if let Some(texture_atlas) = texture_atlases.get(texture_atlas) {
                if Area::from_position_atlas(
                    transform.translation.truncate(),
                    texture_atlas,
                    sprite.index as usize,
                )
                .outside(&screen_rect)
                {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

struct DistanceFromScreen(f32);

/// Edge case for a specific subset of sprite entities:
/// - Are `Wrap` but not `Wrapped` yet
/// - Are outside of the screen
/// - Move further from the screen
/// The latter condition is check by tagging the suspected entity with a
/// `DistanceFromScreen` component which stored the distance between the entity
/// and the screen.
/// At second iteration, if the new distance if larger than the one in the tag,
/// we must make an action.
///
/// Current action is despawn.
fn teleport_wrap_non_wrapped_sprite_atlas(
    mut commands: Commands,
    q_projection: Query<&OrthographicProjection, With<WrapCamera>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    query: Query<
        (
            Entity,
            &Handle<TextureAtlas>,
            &Transform,
            &TextureAtlasSprite,
            Option<&DistanceFromScreen>,
        ),
        (With<Wrap>, Without<Ghost>, Without<Wrapped>),
    >,
) {
    if let Ok(projection) = q_projection.single() {
        let screen_rect = Area::from_projection(projection);
        for (entity, texture_atlas, transform, sprite, last_distance) in query.iter() {
            let position = transform.translation.truncate();

            if let Some(texture_atlas) = texture_atlases.get(texture_atlas) {
                if Area::from_position_atlas(position, texture_atlas, sprite.index as usize)
                    .outside(&Area::from_projection(projection))
                {
                    let distance = screen_rect.distance_squared(position);
                    if let Some(last_distance) = last_distance {
                        if distance >= last_distance.0 {
                            commands.entity(entity).despawn();
                        }
                    } else {
                        commands.entity(entity).insert(DistanceFromScreen(distance));
                    }
                } else {
                    commands.entity(entity).remove::<DistanceFromScreen>();
                }
            }
        }
    }
}

/// Edge case for a specific subset of sprite entities:
/// - Are `Wrap` but not `Wrapped` yet
/// - Are outside of the screen
/// - Move further from the screen
/// The latter condition is checked by tagging the suspected entity with a
/// `DistanceFromScreen` component which stores the distance between the entity
/// and the screen.
/// On second iteration, if the new distance is larger than the one in the tag,
/// we must make an action.
///
/// Current action is despawn.
fn teleport_wrap_non_wrapped_sprite(
    mut commands: Commands,
    q_projection: Query<&OrthographicProjection, With<WrapCamera>>,
    query: Query<
        (Entity, &Transform, &Sprite, Option<&DistanceFromScreen>),
        (With<Wrap>, Without<Ghost>, Without<Wrapped>),
    >,
) {
    if let Ok(projection) = q_projection.single() {
        let screen_rect = Area::from_projection(projection);
        for (entity, transform, sprite, last_distance) in query.iter() {
            let position = transform.translation.truncate();
            if Area::new(position, sprite.size).outside(&screen_rect) {
                let distance = screen_rect.distance_squared(position);
                if let Some(last_distance) = last_distance {
                    if distance >= last_distance.0 {
                        commands.entity(entity).despawn();
                    }
                } else {
                    commands.entity(entity).insert(DistanceFromScreen(distance));
                }
            } else {
                commands.entity(entity).remove::<DistanceFromScreen>();
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
    if let Ok(projection) = q_projection.single() {
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

fn make_ghost_sprite_index(
    q_targets: Query<(Entity, &TextureAtlasSprite)>,
    mut q_ghosts: Query<&mut Ghost>,
) {
    for mut ghost in q_ghosts.iter_mut() {
        if let Ok(sprite) = q_targets.get_component::<TextureAtlasSprite>(ghost.target) {
            ghost.index = Some(sprite.index);
        }
    }
}

/// Recalculate the position of all ghosts according to their target.
/// It does not directly changes the transform, but configures a shift+rotation
/// information that is then used by `move_ghosts`
fn make_ghost_transforms(
    q_projection: Query<&OrthographicProjection, With<WrapCamera>>,
    q_targets: Query<(Entity, &Transform)>,
    mut q_ghosts: Query<&mut Ghost>,
) {
    if let Ok(projection) = q_projection.single() {
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
fn set_ghost_transforms(mut query: Query<(&mut Transform, &Ghost)>) {
    for (mut transform, ghost) in query.iter_mut() {
        transform.translation = ghost.shift;
        transform.rotation = ghost.rotation;
    }
}

/// Replace the ghosts according to their calculated transformation
fn set_ghost_sprite_index(mut query: Query<(&mut TextureAtlasSprite, &Ghost)>) {
    for (mut sprite, ghost) in query.iter_mut() {
        if let Some(index) = ghost.index {
            sprite.index = index;
        }
    }
}

pub struct WrapPlugin;

impl Plugin for WrapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(teleport_wrapped.system().label(Label::Teleport))
                .with_system(
                    teleport_wrap_non_wrapped_sprite
                        .system()
                        .label(Label::Teleport),
                )
                .with_system(
                    teleport_wrap_non_wrapped_sprite_atlas
                        .system()
                        .label(Label::Teleport),
                )
                .with_system(
                    spawn_ghosts_sprite
                        .system()
                        .label(Label::Spawn)
                        .after(Label::Teleport),
                )
                .with_system(
                    spawn_ghosts_sprite_atlas
                        .system()
                        .label(Label::Spawn)
                        .after(Label::Teleport),
                )
                .with_system(
                    make_ghost_transforms
                        .system()
                        .label(Label::Make)
                        .after(Label::Spawn),
                )
                .with_system(
                    make_ghost_sprite_index
                        .system()
                        .label(Label::Make)
                        .after(Label::Spawn),
                )
                .with_system(set_ghost_transforms.system().after(Label::Make))
                .with_system(set_ghost_sprite_index.system().after(Label::Make))
                .with_system(despawn_ghosts_indirect.system())
                .with_system(despawn_ghosts_direct_sprite.system())
                .with_system(despawn_ghosts_direct_sprite_atlas.system())
                .with_system(auto_unwrap.system())
                .with_system(despawn_unwrapped_sprite.system())
                .with_system(despawn_unwrapped_sprite_atlas.system()),
        );
    }
}
