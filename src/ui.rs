use crate::{AppState, NoWrapProtection, PlayerLifes, PlayerTexture, WrapCamera, PLAYER_LIFES_MAX};
use bevy::{
    app::{AppBuilder, Plugin},
    core::{Time, Timer},
    ecs::{
        entity::Entity,
        query::With,
        schedule::SystemSet,
        system::{Commands, IntoSystem, Query, Res},
    },
    math::Vec3,
    render::camera::OrthographicProjection,
    sprite::{entity::SpriteSheetBundle, TextureAtlasSprite},
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
    lifes: Res<PlayerLifes>,
) {
    for (t, token, mut sprite, delay) in q_tokens.iter_mut() {
        if token.0 >= lifes.0 {
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

fn create_ui(mut commands: Commands, player_texture: Res<PlayerTexture>) {
    for life in 0..PLAYER_LIFES_MAX {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: player_texture.0.clone(),
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

fn dispose_ui(mut commands: Commands, query: Query<Entity, With<LifeToken>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(despawn_life_tokens.system())
                .with_system(position_life_tokens.system()),
        )
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(create_ui.system()))
        .add_system_set(SystemSet::on_exit(AppState::Game).with_system(dispose_ui.system()));
    }
}
