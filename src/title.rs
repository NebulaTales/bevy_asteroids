use crate::{AppState, Score};
use bevy::{
    app::{AppBuilder, Plugin},
    asset::AssetServer,
    ecs::{
        query::With,
        schedule::{State, SystemSet},
        system::{Commands, IntoSystem, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, Input},
    math::Rect,
    render::{color::Color, draw::Visible},
    text::{
        prelude::{HorizontalAlign, VerticalAlign},
        Text, TextAlignment, TextStyle,
    },
    ui::{entity::TextBundle, PositionType, Style, Val},
};

pub struct TitlePlugin;

#[derive(Default)]
struct ScoreCounter {
    _highscore: bool,
}

fn launch_game(keyboard: Res<Input<KeyCode>>, mut state: ResMut<State<AppState>>) {
    if keyboard.just_released(KeyCode::Return) {
        state.push(AppState::Game).unwrap();
    }
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

fn enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        // 2d camera
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
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
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

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(AppState::Title)
            .add_system_set(SystemSet::on_enter(AppState::Title).with_system(enter.system()))
            .add_system_set(SystemSet::on_update(AppState::Title).with_system(launch_game.system()))
            .add_system_set(
                SystemSet::on_pause(AppState::Title).with_system(display_score_counter.system()),
            )
            .add_system_set(
                SystemSet::on_inactive_update(AppState::Title)
                    .with_system(update_score_counter.system()),
            );
    }
}
