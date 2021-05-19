use crate::{AppState, GameFont, Score};
use bevy::{
    app::{AppBuilder, Plugin},
    ecs::{
        entity::Entity,
        query::With,
        schedule::{State, SystemSet},
        system::{Commands, IntoSystem, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, Input},
    math::Rect,
    render::color::Color,
    text::{Text, TextSection, TextStyle},
    ui::{entity::TextBundle, AlignSelf, PositionType, Style, Val},
};

pub struct TitlePlugin;

struct Title;

fn launch_game(keyboard: Res<Input<KeyCode>>, mut state: ResMut<State<AppState>>) {
    if keyboard.just_released(KeyCode::Space) {
        state.push(AppState::Game).unwrap();
    }
}

fn remove_title(mut commands: Commands, query: Query<Entity, With<Title>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

fn add_score_title(mut commands: Commands, score: Res<Score>, font: Res<GameFont>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(50.),
                    bottom: Val::Percent(50.),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: format!("Score: {}\n", score.current),
                        style: TextStyle {
                            font: font.0.clone(),
                            font_size: 120.,
                            color: Color::ORANGE_RED,
                        },
                    },
                    TextSection {
                        value: format!("Highest: {}\n", score.highest),
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

fn add_title(mut commands: Commands, font: Res<GameFont>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(50.),
                    bottom: Val::Percent(25.),
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

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(AppState::Title)
            .add_system_set(SystemSet::on_enter(AppState::Title).with_system(add_title.system()))
            .add_system_set(SystemSet::on_update(AppState::Title).with_system(launch_game.system()))
            .add_system_set(
                SystemSet::on_resume(AppState::Title)
                    .with_system(add_score_title.system())
                    .with_system(add_title.system()),
            )
            .add_system_set(
                SystemSet::on_pause(AppState::Title).with_system(remove_title.system()),
            );
    }
}
