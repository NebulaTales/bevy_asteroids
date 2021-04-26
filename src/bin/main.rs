use asteroid::{AsteroidsGamePlugins, Game};
use bevy::{
    app::App,
    asset::AssetServer,
    ecs::{
        query::With,
        system::{Commands, IntoSystem, Query, Res},
    },
    math::Vec3,
    render::color::Color,
    render::pass::ClearColor,
    text::{
        prelude::{HorizontalAlign, VerticalAlign},
        Text, Text2dBundle, TextAlignment, TextStyle,
    },
    transform::components::Transform,
    DefaultPlugins,
};

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(
            40.0 / 255.0,
            42.0 / 255.0,
            54.0 / 255.0,
        )))
        .add_plugins(DefaultPlugins)
        .add_plugins(AsteroidsGamePlugins)
        .add_startup_system(setup.system())
        //.add_system(update_entity_count.exclusive_system())
        .add_system(update_lifes_count.system())
        .run();
}

struct EntityCounter;
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        .insert(EntityCounter);
}

fn update_lifes_count(game: Res<Game>, mut q: Query<&mut Text, With<EntityCounter>>) {
    if let Ok(mut label) = q.single_mut() {
        label.sections[0].value = game.lifes.to_string();
    }
}
