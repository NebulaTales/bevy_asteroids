use crate::Ghost;
use bevy::{
    app::{AppBuilder, Plugin},
    ecs::{
        entity::Entity,
        system::{Commands, IntoSystem, Query},
    },
    math::Vec2,
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
        (Shape2D::Rectangle(extends_a), Shape2D::Circle(radius_b)) => {
            let distance = (position_b + collider_b.position) - (position_a + collider_a.position);
            let clamped = Vec2::new(
                distance.x.clamp(-extends_a.x, extends_a.x),
                distance.y.clamp(-extends_a.y, extends_a.y),
            );
            let closest = position_a + clamped;

            closest.distance_squared(position_b) < *radius_b
        }
        (Shape2D::Circle(_), Shape2D::Rectangle(_)) => {
            check(collider_b, position_b, collider_a, position_a)
        }
    }
}

// TODO Ghost management should be in wrapping plugin
fn transform_based_check(
    mut commands: Commands,
    q_sources: Query<(Entity, &Collider2D, &LayerMask, &Transform, Option<&Ghost>)>,
    q_targets: Query<(&Collider2D, &CollisionMask, &Transform)>,
) {
    for (source, source_collider, source_layer_mask, source_transform, source_ghost) in
        q_sources.iter()
    {
        for (target_collider, target_collision_mask, target_transform) in q_targets.iter() {
            if source_layer_mask.0 & target_collision_mask.0 > 0u8 {
                if check(
                    source_collider,
                    source_transform.translation.into(),
                    target_collider,
                    target_transform.translation.into(),
                ) {
                    let entity = if let Some(ghost) = source_ghost {
                        ghost.target
                    } else {
                        source
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
        app.add_system(transform_based_check.system());
    }
}
