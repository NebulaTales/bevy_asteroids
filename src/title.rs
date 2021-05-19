use crate::{AppState, Score};
use bevy::{
    app::{AppBuilder, Plugin},
    asset::{AssetServer, Handle},
    ecs::{
        entity::Entity,
        query::With,
        schedule::{State, SystemSet},
        system::{Commands, IntoSystem, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, Input},
    math::Rect,
    render::{color::Color, draw::Visible},
    text::{
        prelude::{HorizontalAlign, VerticalAlign},
        Font, Text, TextAlignment, TextSection, TextStyle,
    },
    ui::{entity::TextBundle, AlignSelf, PositionType, Style, Val},
};

pub struct TitlePlugin;

#[derive(Default)]
struct ScoreCounter {
    _highscore: bool,
}

struct Title;
struct GameFont(Handle<Font>);

fn launch_game(keyboard: Res<Input<KeyCode>>, mut state: ResMut<State<AppState>>) {
    if keyboard.just_released(KeyCode::Space) {
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

fn remove_title(mut commands: Commands, query: Query<Entity, With<Title>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

fn enter(mut commands: Commands, font: Res<GameFont>) {
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

fn add_title(mut commands: Commands, font: Res<GameFont>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(50.),
                    bottom: Val::Percent(50.),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Asteroid\n".into(),
                        style: TextStyle {
                            font: font.0.clone(),
                            font_size: 120.,
                            color: Color::ORANGE_RED,
                        },
                    },
                    TextSection {
                        value: "Press Space".into(),
                        style: TextStyle {
                            font: font.0.clone(),
                            font_size: 42.,
                            color: Color::BLUE,
                        },
                    },
                ],
                alignment: Default::default(),
            },
            ..Default::default()
        })
        .insert(Title);
}

fn prepare_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameFont(asset_server.load("fonts/FiraSans-Bold.ttf")));
}

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(AppState::Title)
            .add_startup_system(prepare_resources.system())
            .add_system_set(
                SystemSet::on_enter(AppState::Title)
                    .with_system(add_title.system())
                    .with_system(enter.system()),
            )
            .add_system_set(SystemSet::on_update(AppState::Title).with_system(launch_game.system()))
            .add_system_set(SystemSet::on_resume(AppState::Title).with_system(add_title.system()))
            .add_system_set(
                SystemSet::on_pause(AppState::Title)
                    .with_system(display_score_counter.system())
                    .with_system(remove_title.system()),
            )
            .add_system_set(
                SystemSet::on_inactive_update(AppState::Title)
                    .with_system(update_score_counter.system()),
            );
    }
}
