use bevy::{
    app::{AppBuilder, Plugin},
    ecs::system::{Commands, IntoSystem},
};

pub const SCORE_BIG_ASTEROID: u16 = 5;
pub const SCORE_SMALL_ASTEROID: u16 = 10;
pub const SCORE_TINY_ASTEROID: u16 = 15;
pub const SCORE_SAUCER: u16 = 100;

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

pub struct ScorePlugin;

pub fn startup(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system());
    }
}
