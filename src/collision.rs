use crate::Ghost;
use bevy::{
    app::{AppBuilder, Plugin},
    ecs::{
        entity::Entity,
        system::{Commands, IntoSystem, Query},
    },
    math::{Vec2, Vec3},
    render::camera::OrthographicProjection,
    sprite::{collide_aabb::collide, Sprite},
    transform::components::Transform,
};

#[derive(Clone, Copy)]
pub struct LayerMask(pub u8);

#[derive(Clone, Copy)]
pub struct CollisionMask(pub u8);

pub const PLAYER: u8 = 0b00000001;
pub const OBSTACLE: u8 = 0b00000010;

//pub fn collide(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> Option<Collision> {
pub fn square_in_screen(screen: &OrthographicProjection, position: Vec3, size: Vec2) -> bool {
    let screen_size = Vec2::new(screen.right - screen.left, screen.top - screen.bottom);
    let screen_translation = Vec3::new(
        (screen.right + screen.left) / 2.0,
        (screen.top + screen.bottom) / 2.0,
        0.0,
    );

    let a_min = screen_translation.truncate() - screen_size / 2.0;
    let a_max = screen_translation.truncate() + screen_size / 2.0;

    let b_min = position.truncate() - size / 2.0;
    let b_max = position.truncate() + size / 2.0;

    // check to see if the two rectangles are intersecting
    a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
}

pub fn layer_check(
    mut commands: Commands,
    q_projection: Query<&OrthographicProjection>,
    source_query: Query<(Entity, &Sprite, &Transform, &CollisionMask, Option<&Ghost>)>,
    target_query: Query<(Entity, &Sprite, &Transform, &LayerMask)>,
) {
    for screen in q_projection.iter() {
        for (src, src_sprite, src_transform, src_mask, src_ghost) in source_query.iter() {
            for (_, tgt_sprite, tgt_transform, tgt_mask) in target_query.iter() {
                let objects_in_screen =
                    square_in_screen(screen, src_transform.translation, src_sprite.size)
                        && square_in_screen(screen, tgt_transform.translation, tgt_sprite.size);

                if objects_in_screen {
                    let objects_collide = collide(
                        src_transform.translation,
                        src_sprite.size,
                        tgt_transform.translation,
                        tgt_sprite.size,
                    )
                    .is_some();

                    if objects_collide {
                        if src_mask.0 & tgt_mask.0 > 0u8 {
                            let entity = if let Some(ghost) = src_ghost {
                                ghost.target
                            } else {
                                src
                            };
                            commands.despawn(entity);
                        }
                    }
                }
            }
        }
    }
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(layer_check.system());
    }
}
