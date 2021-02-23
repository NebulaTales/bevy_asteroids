use bevy::{
    app::{AppBuilder, Plugin},
    ecs::IntoSystem,
    render::{color::Color, pass::ClearColor},
};

pub mod component;
mod system;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ClearColor(Color::rgb(0.1, 0.0, 0.2)))
            .add_system(system::acceleration.system())
            .add_system(system::thrust.system())
            .add_system(system::floor_velocity.system())
            .add_system(system::velocity.system())
            .add_system(system::collision.system())
            .add_system(system::friction.system());
    }
}
