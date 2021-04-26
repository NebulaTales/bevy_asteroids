use crate::Game;
use bevy::{
    app::{AppBuilder, Plugin},
    asset::AssetServer,
    ecs::{
        query::With,
        system::{Commands, IntoSystem, Query, Res},
    },
    math::Vec3,
    render::color::Color,
    text::{
        prelude::{HorizontalAlign, VerticalAlign},
        Text, Text2dBundle, TextAlignment, TextStyle,
    },
    transform::components::Transform,
};

struct DebugCounter;
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        // 2d camera
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "N/A",
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
            transform: Transform::from_translation(Vec3::new(0.0, 280.0, 100.0)),
            ..Default::default()
        })
        .insert(DebugCounter);
}

fn update_lifes_count(game: Res<Game>, mut q: Query<&mut Text, With<DebugCounter>>) {
    if let Ok(mut label) = q.single_mut() {
        label.sections[0].value = game.lifes.to_string();
    }
}
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system(update_lifes_count.system());
    }
}
