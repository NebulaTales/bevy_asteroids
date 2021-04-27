use crate::{Game, NoWrapProtection, WrapCamera};
use bevy::{
    app::{AppBuilder, Plugin},
    asset::{AssetServer, Assets},
    ecs::{
        query::With,
        system::{Commands, IntoSystem, Query, Res, ResMut},
    },
    math::{Vec2, Vec3},
    render::{camera::OrthographicProjection, color::Color},
    sprite::{entity::SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    text::{
        prelude::{HorizontalAlign, VerticalAlign},
        Text, Text2dBundle, TextAlignment, TextStyle,
    },
    transform::components::Transform,
};

struct LifeToken(u8);

const TOKEN_MARGIN: f32 = 25.0;
// For now position according to cursor
fn position_life_tokens(
    mut q_tokens: Query<(&LifeToken, &mut Transform)>,
    q_projection: Query<&OrthographicProjection, With<WrapCamera>>,
) {
    if let Ok(projection) = q_projection.single() {
        for (token, mut transform) in q_tokens.iter_mut() {
            let x = projection.left + TOKEN_MARGIN + token.0 as f32 * (TOKEN_MARGIN / 2.0 + 32.0);
            let y = projection.bottom + TOKEN_MARGIN;
            transform.translation = Vec3::new(x, y, 0.0);
        }
    }
}

struct DebugCounter;
fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Create 3 mini ships sprites that will represent lifes
    let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("sprites/ship.png"),
        Vec2::new(64.0, 64.0),
        13,
        1,
    ));

    for life in 0..3 {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas.clone(),
                transform: Transform::from_scale(Vec3::new(0.5, 0.5, 1.0)),
                sprite: TextureAtlasSprite {
                    index: 12,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(LifeToken(life))
            .insert(NoWrapProtection);
    }

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
        app.add_system(position_life_tokens.system());
    }
}
