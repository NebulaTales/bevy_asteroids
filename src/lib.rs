use bevy::app::{PluginGroup, PluginGroupBuilder};

mod collision;
mod controls;
mod movement;
mod startup;
mod utils;

pub use collision::{CollisionMask, CollisionPlugin, LayerMask, OBSTACLE, PLAYER};
pub use controls::{ControlsPlugin, PlayerControlled};
pub use movement::{Acceleration, Friction, MovementPlugin, Thrust, Velocity};
pub use startup::StartupPlugin;
pub use utils::DelayedAdd;

pub struct AsteroidPlugins;

impl PluginGroup for AsteroidPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(StartupPlugin);
        group.add(CollisionPlugin);
        group.add(MovementPlugin);
        group.add(ControlsPlugin);
    }
}
