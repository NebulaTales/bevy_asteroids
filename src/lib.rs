use bevy::{
    app::{AppBuilder, Plugin, PluginGroup, PluginGroupBuilder},
    ecs::IntoSystem,
};

mod collision;
mod controls;
mod movement;
mod startup;
mod utils;

pub use collision::{CollisionMask, LayerMask, OBSTACLE, PLAYER};
pub use controls::PlayerControlled;
pub use movement::{Acceleration, Friction, Thrust, Velocity};
pub use utils::DelayedAdd;

pub struct AsteroidPlugins;
pub struct StartupPlugin;
pub struct CollisionPlugin;
pub struct MovementPlugin;
pub struct ControlsPlugin;

impl PluginGroup for AsteroidPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(StartupPlugin);
        group.add(CollisionPlugin);
        group.add(MovementPlugin);
        group.add(ControlsPlugin);
    }
}

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup::player.system())
            .add_startup_system(startup::asteroids.system())
            .add_startup_system(startup::game.system())
            .add_startup_system(startup::environment.system());
    }
}

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(collision::layer_check.system());
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(movement::acceleration.system())
            .add_system(movement::floor_velocity.system())
            .add_system(movement::velocity.system())
            .add_system(movement::friction.system());
    }
}

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(controls::keyboard.system())
            .add_system(utils::delayed_add::<controls::PlayerControlled>.system());
    }
}
