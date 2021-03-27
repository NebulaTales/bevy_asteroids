use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    math::Vec2,
};

mod asteroids;
mod collision;
mod controls;
mod fire;
mod movement;
mod startup;
mod utils;
mod wrap;

pub use asteroids::RulesPlugin;
pub use collision::{Collider2D, CollisionEvent, CollisionLayer, CollisionMask, CollisionPlugin};
pub use controls::{ControlsPlugin, PlayerControlled};
pub use fire::{FireAngleError, FirePlugin, Firing};
pub use movement::{Acceleration, Friction, MovementPlugin, Thrust, Velocity};
pub use startup::StartupPlugin;
pub use utils::DelayedAdd;
pub use wrap::{Ghost, Wrap, WrapCamera, WrapPlugin, Wrapped};

pub struct AsteroidPlugins;

pub const PLAYER: u8 = 0b00000001;
pub const OBSTACLE: u8 = 0b00000010;
pub const AMMO: u8 = 0b00000100;

pub enum Shape2D {
    Rectangle(Vec2),
    Circle(f32),
}

impl Default for Shape2D {
    fn default() -> Self {
        Shape2D::Rectangle(Vec2::new(1.0, 1.0))
    }
}

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
