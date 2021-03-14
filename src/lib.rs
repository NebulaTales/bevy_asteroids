use bevy::app::{PluginGroup, PluginGroupBuilder};

mod collision;
mod controls;
mod fire;
mod movement;
mod rules;
mod startup;
mod utils;
mod wrap;

pub use collision::{CollisionMask, CollisionPlugin, LayerMask, OBSTACLE, PLAYER};
pub use controls::{ControlsPlugin, PlayerControlled};
pub use fire::{FirePlugin, FireSpawner};
pub use movement::{Acceleration, Friction, MovementPlugin, Thrust, Velocity};
pub use rules::RulesPlugin;
pub use startup::StartupPlugin;
pub use utils::DelayedAdd;
pub use wrap::{Ghost, Wrap, WrapCamera, WrapPlugin, Wrapped};

pub struct AsteroidPlugins;

impl PluginGroup for AsteroidPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(CollisionPlugin);
        group.add(ControlsPlugin);
        group.add(FirePlugin);
        group.add(MovementPlugin);
        group.add(RulesPlugin);
        group.add(StartupPlugin);
        group.add(WrapPlugin);
    }
}
