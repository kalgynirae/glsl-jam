use bevy::prelude::*;

pub struct MusicPlugin;
impl Plugin for MusicPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(MusicTimer(Timer::from_seconds(38.70967741935484, true), true)).add_system(play.system());
    }
}

struct MusicTimer(Timer, bool);

fn play(
    asset_server: ResMut<AssetServer>,
    audio: Res<Audio>,
    time: Res<Time>,
    mut timer: ResMut<MusicTimer>,
) {
    if !timer.1 && !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    timer.1 = false;
    let music = asset_server.load("GLSL Jam.mp3");
    audio.play(music);
    timer.0.reset();
}
