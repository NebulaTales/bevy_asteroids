/// Collision system
/// Collisions work with a Layer/Mask design in mind (similar to Godot's):
///
/// A _layer_ is an identifier stored in a bit field (`u8`).
/// When an entity has a `CollisionLayer` components, each bit of this value
/// represent a different layer the entity belong to.
///
/// A `CollisionMask` is a component representing the set of layers an entity
/// can collide with.
///
/// When a collision is detected between a _source_ (the one holding the
/// `CollisionMask` component) and a _target_ (the one with a `CollisionLayer`),
/// a `CollisionEvent` event is emitted that can be used within other systems.
///
use crate::{Ghost, Shape2D};
use bevy::{
    app::{AppBuilder, EventWriter, Plugin},
    ecs::{
        entity::Entity,
        system::{IntoSystem, Query},
    },
    math::Vec2,
    transform::components::Transform,
};

/// Defines the layers an entity belongs to
#[derive(Clone, Copy)]
pub struct CollisionLayer(pub u8);

/// Set the layers an entity interacts with
#[derive(Clone, Copy)]
pub struct CollisionMask(pub u8);

pub struct CollisionEvent {
    pub source: Entity,
    pub target: Entity,
    pub layer: u8,
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
            let square_center = position_a - collider_a.position;
            let circle_center = position_b + collider_b.position;
            circle_center.distance_squared(
                Vec2::new(
                    (circle_center.x - square_center.x).clamp(-extends_a.x, extends_a.x),
                    (circle_center.y - square_center.y).clamp(-extends_a.y, extends_a.y),
                ) + square_center,
            ) < radius_b.powf(2.0)
        }
        (Shape2D::Circle(_), Shape2D::Rectangle(_)) => {
            check(collider_b, position_b, collider_a, position_a)
        }
    }
}

// TODO Ghost management should be in wrapping plugin
fn transform_based_check(
    mut events: EventWriter<CollisionEvent>,
    q_sources: Query<(
        Entity,
        &Collider2D,
        &CollisionMask,
        &Transform,
        Option<&Ghost>,
    )>,
    q_targets: Query<(Entity, &Collider2D, &CollisionLayer, &Transform)>,
) {
    for (source, source_collider, source_collision_mask, source_transform, source_ghost) in
        q_sources.iter()
    {
        for (target, target_collider, target_collision_layer, target_transform) in q_targets.iter()
        {
            let layer = source_collision_mask.0 & target_collision_layer.0;
            if layer > 0u8 {
                if check(
                    source_collider,
                    source_transform.translation.into(),
                    target_collider,
                    target_transform.translation.into(),
                ) {
                    let source = if let Some(ghost) = source_ghost {
                        ghost.target
                    } else {
                        source
                    };

                    events.send(CollisionEvent {
                        source,
                        target,
                        layer,
                    });
                }
            }
        }
    }
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<CollisionEvent>()
            .add_system(transform_based_check.system());
    }
}
