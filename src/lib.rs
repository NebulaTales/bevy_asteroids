use bevy::prelude::*;

pub mod component;
mod system;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ClearColor(Color::rgb(0.1, 0.0, 0.2)))
            .add_system(system::acceleration_system.system())
            .add_system(system::thrust_system.system())
            .add_system(system::floor_velocity_system.system())
            .add_system(system::velocity_system.system())
            .add_system(system::friction_system.system());
    }
}
