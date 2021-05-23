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
}

pub struct SoundEffects {
    pub fire: Handle<AudioSource>,
}

pub fn prepare_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let background = asset_server.load("audio/background.mp3");
    let fire = asset_server.load("audio/fire.wav");

    let channels = AudioChannels {
        background: AudioChannel::new("music".to_owned()),
        fx: AudioChannel::new("fx".to_owned()),
    };

    audio.play_looped_in_channel(background, &channels.background);

    commands.insert_resource(SoundEffects { fire });
    commands.insert_resource(channels);
}

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(prepare_resources.system());
    }
}
