use bevy::{
    app::{AppBuilder, Plugin},
    asset::{AssetServer, Handle},
    ecs::system::{Commands, IntoSystem, Res},
};

use bevy_kira_audio::{Audio, AudioChannel, AudioSource};
pub struct AudioPlugin;

pub struct AudioChannels {
    pub background: AudioChannel,
    pub fx: AudioChannel,
    pub fx_ufo: AudioChannel,
}

pub struct SoundEffects {
    pub fire: Handle<AudioSource>,
    pub ufo: Handle<AudioSource>,
    pub boom: Handle<AudioSource>,
}

pub fn prepare_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let channels = AudioChannels {
        background: AudioChannel::new("music".to_owned()),
        fx: AudioChannel::new("fx".to_owned()),
        fx_ufo: AudioChannel::new("fx_ufo".to_owned()),
    };
    audio.set_volume_in_channel(2.0, &channels.fx_ufo);
    audio.set_volume_in_channel(0.1, &channels.fx);

    audio.play_looped_in_channel(
        asset_server.load("audio/background.mp3"),
        &channels.background,
    );

    commands.insert_resource(SoundEffects {
        fire: asset_server.load("audio/fire.wav"),
        ufo: asset_server.load("audio/ufo.wav"),
        boom: asset_server.load("audio/boom.wav"),
    });
    commands.insert_resource(channels);
}

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(prepare_resources.system());
    }
}
