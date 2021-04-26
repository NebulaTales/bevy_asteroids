use asteroid::AsteroidsGamePlugins;
use bevy::{app::App, render::color::Color, render::pass::ClearColor, DefaultPlugins};

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(
            40.0 / 255.0,
            42.0 / 255.0,
            54.0 / 255.0,
        )))
        .add_plugins(DefaultPlugins)
        .add_plugins(AsteroidsGamePlugins)
        .run();
}
