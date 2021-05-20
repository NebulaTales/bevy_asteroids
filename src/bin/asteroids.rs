use asteroid::AsteroidsGamePlugins;
use bevy::{app::App, render::color::Color, render::pass::ClearColor, DefaultPlugins};

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(
            10.0 / 255.0,
            10.0 / 255.0,
            20.0 / 255.0,
        )))
        .add_plugins(DefaultPlugins)
        .add_plugins(AsteroidsGamePlugins)
        .run();
}
