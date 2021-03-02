use bevy::{
    app::{AppBuilder, Plugin},
    ecs::{Commands, Entity, IntoSystem, Query},
    sprite::{collide_aabb::collide, Sprite},
    transform::components::Transform,
};

pub struct LayerMask(pub u8);
pub struct CollisionMask(pub u8);
pub const PLAYER: u8 = 0b00000001;
pub const OBSTACLE: u8 = 0b00000010;

pub fn layer_check(
    commands: &mut Commands,
    source_query: Query<(Entity, &Sprite, &Transform, &CollisionMask)>,
    target_query: Query<(Entity, &Sprite, &Transform, &LayerMask)>,
) {
    for (src, src_sprite, src_transform, src_mask) in source_query.iter() {
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
                    commands.despawn(src);
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
