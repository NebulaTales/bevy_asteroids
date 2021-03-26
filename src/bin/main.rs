use asteroid::AsteroidPlugins;
use bevy::{
    app::App,
    asset::AssetServer,
    ecs::{
        query::With,
        system::{Commands, IntoExclusiveSystem, IntoSystem, Res},
        world::World,
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
        .add_plugins(AsteroidPlugins)
        .add_startup_system(setup.system())
        .add_system(update_entity_count.exclusive_system())
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

fn update_entity_count(mut world: &mut World) {
    let mut query = world.query_filtered::<&mut Text, With<EntityCounter>>();
    let value = (*world.entities()).len();
    for mut counter in query.iter_mut(&mut world) {
        counter.sections[0].value = value.to_string();
    }
}
