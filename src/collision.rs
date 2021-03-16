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

/// Defines the layers an entity belongs to
#[derive(Clone, Copy)]
pub struct LayerMask(pub u8);

/// Set the layers an entity interacts with
#[derive(Clone, Copy)]
pub struct CollisionMask(pub u8);

pub enum Shape2D {
    Rectangle(Vec2),
    Circle(f32),
}

impl Default for Shape2D {
    fn default() -> Self {
        Shape2D::Rectangle(Vec2::new(1.0, 1.0))
    }
}

#[derive(Default)]
pub struct Collider2D {
    pub shape: Shape2D,
    pub position: Vec2,
}

fn check(
    collider_a: &Collider2D,
    position_a: Vec2,
    collider_b: &Collider2D,
    position_b: Vec2,
) -> bool {
    match (&collider_a.shape, &collider_b.shape) {
        (Shape2D::Rectangle(extends_a), Shape2D::Rectangle(extends_b)) => {
            let a_min = (collider_a.position + position_a) - *extends_a;
            let a_max = (collider_a.position + position_a) + *extends_a;
            let b_min = (collider_b.position + position_b) - *extends_b;
            let b_max = (collider_b.position + position_b) + *extends_b;
            a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
        }
        (Shape2D::Circle(radius_a), Shape2D::Circle(radius_b)) => {
            (collider_a.position + position_a).distance_squared(collider_b.position + position_b)
                < (radius_a + radius_b).powf(2.0)
        }
        _ => false,
    }
}

/// Collision detection system between two entities
/// The system determines a collision between a source and a target
/// The source must have a CollisionMask, which lists all layers it collides with
/// The target must have a LayerMask, which lists the layers it corresponds to
/// Entitie must also have a Transform component, which is used to get the actual
/// positions
fn transform_based_check(
    mut commands: Commands,
    q_sources: Query<(Entity, &Collider2D, &LayerMask, &Transform)>,
    q_targets: Query<(Entity, &Collider2D, &CollisionMask, &Transform)>,
) {
    for (_source, source_collider, source_layer_mask, source_transform) in q_sources.iter() {
        for (target, target_collider, target_collision_mask, target_transform) in q_targets.iter() {
            if source_layer_mask.0 & target_collision_mask.0 > 0u8 {
                if check(
                    source_collider,
                    source_transform.translation.into(),
                    target_collider,
                    target_transform.translation.into(),
                ) {
                    commands.despawn(target);
                }
            }
        }
    }
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(transform_based_check.system());
    }
}
