use bevy::{
    app::{AppBuilder, AppExit, Events, Plugin},
    ecs::system::{Commands, IntoSystem, Res, ResMut},
};

pub const SCORE_BIG_ASTEROID: u16 = 5;
pub const SCORE_SMALL_ASTEROID: u16 = 10;
pub const SCORE_TINY_ASTEROID: u16 = 15;
pub const PLAYER_LIFES_MAX: u8 = 3;

pub struct PlayerLifes(pub u8);

impl Default for PlayerLifes {
    fn default() -> Self {
        PlayerLifes(PLAYER_LIFES_MAX)
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Score {
    pub current: u16,
    pub highest: u16,
}

impl Score {
    pub fn add(&mut self, increment: u16) -> bool {
        self.current += increment;
        if self.current > self.highest {
            self.highest = self.current;
            true
        } else {
            false
        }
    }

    pub fn is_highest(&self) -> bool {
        self.current == self.highest
    }
}

pub struct RulesPlugin;

pub fn startup(mut commands: Commands) {
    commands.insert_resource(Score::default());
    commands.insert_resource(PlayerLifes::default());
}

fn game_over(lifes: Res<PlayerLifes>, score: Res<Score>, mut signal: ResMut<Events<AppExit>>) {
    if lifes.0 == 0 {
        dbg!(*score);
        signal.send(AppExit);
    }
}

impl Plugin for RulesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system(game_over.system());
    }
}
