use bevy::{
    core::{Time, Timer},
    ecs::{Commands, Component, Entity, Mut, Query, Res},
};

pub struct DelayedAdd<T>(pub T, pub Timer)
where
    T: Component + Copy;

pub fn delayed_add<T>(
    time: Res<Time>,
    commands: &mut Commands,
    mut query: Query<(Entity, Mut<DelayedAdd<T>>)>,
) where
    T: Component + Copy,
{
    for (entity, mut add) in query.iter_mut() {
        if add.1.tick(time.delta_seconds()).just_finished() {
            commands.remove_one::<T>(entity).insert_one(entity, add.0);
        }
    }
}
