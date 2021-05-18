use crate::AppState;
use bevy::{
    app::{AppBuilder, Plugin},
    ecs::{
        schedule::State,
        schedule::SystemSet,
        system::{IntoSystem, Res, ResMut},
    },
    input::{keyboard::KeyCode, Input},
};

pub struct TitlePlugin;

fn launch_game(keyboard: Res<Input<KeyCode>>, mut state: ResMut<State<AppState>>) {
    if keyboard.just_released(KeyCode::Return) {
        state.push(AppState::Game).unwrap();
    }
}

fn setup() {
    println!("setup");
}

fn teardown() {
    println!("teardown");
}

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(AppState::Title)
            .add_system_set(SystemSet::on_update(AppState::Title).with_system(launch_game.system()))
            .add_system_set(SystemSet::on_enter(AppState::Title).with_system(setup.system()))
            .add_system_set(SystemSet::on_pause(AppState::Title).with_system(teardown.system()))
            .add_system_set(SystemSet::on_exit(AppState::Title).with_system(teardown.system()));
    }
}
