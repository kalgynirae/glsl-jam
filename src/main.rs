use bevy::prelude::*;

mod background;
mod input;
mod music;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "ld(7*7)".to_string(),
            width: 480.,
            height: 480.,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(background::BackgroundPlugin)
        .add_plugin(music::MusicPlugin)
        .add_system(input::exit_on_q_or_esc.system())
        .run();
}
