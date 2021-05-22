use bevy::{
    app::{AppBuilder, Plugin},
    asset::AssetServer,
    ecs::system::{Commands, IntoSystem, Res},
};

use bevy_kira_audio::Audio;
pub struct AudioPlugin;

pub fn prepare_resources(_: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_looped(asset_server.load("audio/background.mp3"));
}

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(prepare_resources.system());
    }
}
