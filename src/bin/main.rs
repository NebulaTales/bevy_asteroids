use asteroid::AsteroidPlugins;
use bevy::{app::App, DefaultPlugins};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugins(AsteroidPlugins)
        .run();
}
