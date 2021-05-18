use crate::{AppState, Score};
use bevy::{
    app::{AppBuilder, AppExit, Events, Plugin},
    ecs::{
        schedule::{State, SystemSet},
        system::{Commands, IntoSystem, Res, ResMut},
    },
};

pub const PLAYER_LIFES_MAX: u8 = 3;

pub struct PlayerLifes(pub u8);

impl Default for PlayerLifes {
    fn default() -> Self {
        PlayerLifes(PLAYER_LIFES_MAX)
    }
}

pub struct RulesPlugin;

pub fn startup(mut commands: Commands) {
    commands.insert_resource(PlayerLifes::default());
}

fn game_over(
    mut state: ResMut<State<AppState>>,
    lifes: Res<PlayerLifes>,
    score: Res<Score>,
    _: ResMut<Events<AppExit>>,
) {
    if lifes.0 == 0 {
        dbg!(*score);
        //signal.send(AppExit);
        state.pop().unwrap();
    }
}

impl Plugin for RulesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(game_over.system()));
    }
}
