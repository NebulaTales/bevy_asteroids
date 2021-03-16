use bevy::app::{PluginGroup, PluginGroupBuilder};

mod collision;
mod controls;
mod fire;
mod movement;
mod rules;
mod startup;
mod utils;
mod wrap;

pub use collision::{Collider2D, CollisionMask, CollisionPlugin, LayerMask, Shape2D};
pub use controls::{ControlsPlugin, PlayerControlled};
pub use fire::{FireAngleError, FirePlugin, Firing};
pub use movement::{Acceleration, Friction, MovementPlugin, Thrust, Velocity};
pub use rules::RulesPlugin;
pub use startup::StartupPlugin;
pub use utils::DelayedAdd;
pub use wrap::{Ghost, Wrap, WrapCamera, WrapPlugin, Wrapped};

pub struct AsteroidPlugins;

pub const PLAYER: u8 = 0b00000001;
pub const OBSTACLE: u8 = 0b00000010;
pub const AMMO: u8 = 0b00000100;

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
