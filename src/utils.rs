use bevy::{
    core::{Time, Timer},
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query, Res},
    },
};

pub struct DelayedAdd<T>(pub T, pub Timer)
where
    T: Component + Copy;

pub fn delayed_add<T>(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DelayedAdd<T>)>,
) where
    T: Component + Copy,
{
    for (entity, mut add) in query.iter_mut() {
        if add.1.tick(time.delta()).just_finished() {
            commands.entity(entity).remove::<T>().insert(add.0);
        }
    }
}
