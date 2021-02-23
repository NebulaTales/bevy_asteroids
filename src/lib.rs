use bevy::{
    app::{AppBuilder, Plugin},
    ecs::IntoSystem,
};

pub mod component;
mod system;

use component::PlayerControlled;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(system::acceleration.system())
            .add_system(system::keyboard_thrust.system())
            .add_system(system::floor_velocity.system())
            .add_system(system::velocity.system())
            .add_system(system::collision.system())
            .add_system(system::delayed_add::<PlayerControlled>.system())
            .add_system(system::friction.system());
    }
}
