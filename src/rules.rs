use bevy::{
    app::{AppBuilder, AppExit, Events, Plugin},
    ecs::system::{Commands, IntoSystem, Res, ResMut},
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

fn game_over(game: Res<Game>, mut signal: ResMut<Events<AppExit>>) {
    if game.lifes == 0 {
        signal.send(AppExit);
    }
}

impl Plugin for RulesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system(game_over.system());
    }
}
