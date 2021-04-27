use crate::{Game, NoWrapProtection, WrapCamera};
use bevy::{
    app::{AppBuilder, Plugin},
    asset::{AssetServer, Assets},
    core::{Time, Timer},
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, IntoSystem, Query, Res, ResMut},
    },
    math::{Vec2, Vec3},
    render::camera::OrthographicProjection,
    sprite::{entity::SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    transform::components::Transform,
};
use std::time::Duration;

struct LifeToken(u8);
struct LifeTokenAnimDelay(Timer);

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

fn despawn_life_tokens(
    mut commands: Commands,
    time: Res<Time>,
    mut q_tokens: Query<(
        Entity,
        &LifeToken,
        &mut TextureAtlasSprite,
        Option<&mut LifeTokenAnimDelay>,
    )>,
    game: Res<Game>,
) {
    for (t, token, mut sprite, delay) in q_tokens.iter_mut() {
        if token.0 >= game.lifes {
            if let Some(mut delay) = delay {
                if delay.0.tick(time.delta()).just_finished() {
                    if sprite.index > 0 {
                        sprite.index -= 1;
                    } else {
                        commands.entity(t).despawn();
                    }
                }
            } else {
                commands.entity(t).insert(LifeTokenAnimDelay(Timer::new(
                    Duration::from_millis(50),
                    true,
                )));
            }
        }
    }
}

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
                    index: 11,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(LifeToken(life))
            .insert(NoWrapProtection);
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system(despawn_life_tokens.system())
            .add_system(position_life_tokens.system());
    }
}
