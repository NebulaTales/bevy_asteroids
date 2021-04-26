use bevy::{
    app::{AppBuilder, Plugin},
    ecs::system::{Commands, IntoSystem},
};

pub struct Game {
    pub lifes: u8,
}

impl Default for Game {
    fn default() -> Self {
        Game { lifes: 3 }
    }
}

pub struct RulesPlugin;

pub fn startup(mut commands: Commands) {
    commands.insert_resource(Game::default());
}

impl Plugin for RulesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system());
    }
}
