use bevy::{
    app::{AppBuilder, Plugin, PluginGroup, PluginGroupBuilder},
    ecs::system::{Commands, IntoSystem},
    math::Vec2,
    render::entity::OrthographicCameraBundle,
};

mod asteroids;
mod collision;
mod controls;
mod fire;
mod movement;
mod player;
mod rules;
mod ui;
mod wrap;

pub use asteroids::AsteroidsPlugin;
pub use collision::{Collider2D, CollisionEvent, CollisionLayer, CollisionMask, CollisionPlugin};
pub use controls::{ControlLocked, ControlsPlugin, PlayerControlled};
pub use fire::{FireAngleError, FirePlugin, Firing};
pub use movement::{Acceleration, Friction, MovementPlugin, Thrust, Velocity};
pub use player::PlayerPlugin;
pub use rules::{Game, RulesPlugin};
pub use ui::UIPlugin;
pub use wrap::{Ghost, Wrap, WrapCamera, WrapPlugin, Wrapped};

pub struct AsteroidsGamePlugins;

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

struct BasePlugin;

pub fn game(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(WrapCamera);
}

impl Plugin for BasePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(game.system());
    }
}

impl PluginGroup for AsteroidsGamePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(BasePlugin);
        group.add(CollisionPlugin);
        group.add(ControlsPlugin);
        group.add(FirePlugin);
        group.add(MovementPlugin);
        group.add(UIPlugin);
        group.add(RulesPlugin);
        group.add(AsteroidsPlugin);
        group.add(PlayerPlugin);
        group.add(WrapPlugin);
    }
}
