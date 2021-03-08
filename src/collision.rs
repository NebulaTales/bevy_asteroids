use crate::Ghost;
use bevy::{
    app::{AppBuilder, Plugin},
    ecs::{
        entity::Entity,
        system::{Commands, IntoSystem, Query},
    },
    sprite::{collide_aabb::collide, Sprite},
    transform::components::Transform,
};

#[derive(Clone, Copy)]
pub struct LayerMask(pub u8);

#[derive(Clone, Copy)]
pub struct CollisionMask(pub u8);

pub const PLAYER: u8 = 0b00000001;
pub const OBSTACLE: u8 = 0b00000010;

pub fn layer_check(
    mut commands: Commands,
    source_query: Query<(Entity, &Sprite, &Transform, &CollisionMask, Option<&Ghost>)>,
    target_query: Query<(Entity, &Sprite, &Transform, &LayerMask)>,
) {
    for (src, src_sprite, src_transform, src_mask, src_ghost) in source_query.iter() {
        for (_, tgt_sprite, tgt_transform, tgt_mask) in target_query.iter() {
            if collide(
                src_transform.translation,
                src_sprite.size,
                tgt_transform.translation,
                tgt_sprite.size,
            )
            .is_some()
            {
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

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(layer_check.system());
    }
}
