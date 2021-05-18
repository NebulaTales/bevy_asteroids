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

pub fn initialize_lifes(mut commands: Commands) {
    commands.insert_resource(PlayerLifes::default());
}

pub fn remove_lifes(mut commands: Commands) {
    commands.remove_resource::<PlayerLifes>();
}

fn game_over(
    mut state: ResMut<State<AppState>>,
    lifes: Res<PlayerLifes>,
    score: Res<Score>,
    _: ResMut<Events<AppExit>>,
) {
    dbg!(lifes.0);
    if lifes.0 == 0 {
        dbg!(*score);
        //signal.send(AppExit);
        state.pop().unwrap();
    }
}

impl Plugin for RulesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(game_over.system()))
            .add_system_set(
                SystemSet::on_enter(AppState::Game).with_system(initialize_lifes.system()),
            )
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(remove_lifes.system()));
    }
}
