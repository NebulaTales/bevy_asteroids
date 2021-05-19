use crate::{AppState, GameFont};
use bevy::{
    app::{AppBuilder, Plugin},
    ecs::{
        query::With,
        schedule::SystemSet,
        system::{Commands, IntoSystem, Query, Res, ResMut},
    },
    math::Rect,
    render::{color::Color, draw::Visible},
    text::{
        prelude::{HorizontalAlign, VerticalAlign},
        Text, TextAlignment, TextStyle,
    },
    ui::{entity::TextBundle, PositionType, Style, Val},
};

pub const SCORE_BIG_ASTEROID: u16 = 5;
pub const SCORE_SMALL_ASTEROID: u16 = 10;
pub const SCORE_TINY_ASTEROID: u16 = 15;
pub const SCORE_SAUCER: u16 = 100;

#[derive(Default)]
struct ScoreCounter {
    _highscore: bool,
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

#[derive(Default, Debug, Copy, Clone)]
pub struct Score {
    pub current: u16,
    pub highest: u16,
}

pub fn startup(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn reset_score(mut score: ResMut<Score>) {
    score.current = 0;
}

fn update_score_counter(score: Res<Score>, mut q: Query<&mut Text, With<ScoreCounter>>) {
    if let Ok(mut label) = q.single_mut() {
        label.sections[0].value = score.current.to_string();
    }
}

fn display_score_counter(mut q_score_counters: Query<&mut Visible, With<ScoreCounter>>) {
    for mut visible in q_score_counters.iter_mut() {
        visible.is_visible = true;
    }
}

fn add_score_counter(mut commands: Commands, font: Res<GameFont>) {
    commands
        .spawn_bundle(TextBundle {
            visible: Visible {
                is_visible: false,
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Percent(0.0),
                    left: Val::Percent(0.0),
                    ..Default::default()
                },

                ..Default::default()
            },
            text: Text::with_section(
                "0",
                TextStyle {
                    font: font.0.clone(),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        })
        .insert(ScoreCounter::default());
}

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system_set(
                SystemSet::on_enter(AppState::Game)
                    .with_system(add_score_counter.system())
                    .with_system(reset_score.system()),
            )
            .add_system_set(
                SystemSet::on_pause(AppState::Game).with_system(display_score_counter.system()),
            )
            .add_system_set(
                SystemSet::on_inactive_update(AppState::Title)
                    .with_system(update_score_counter.system()),
            );
    }
}
