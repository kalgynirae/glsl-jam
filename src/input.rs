use bevy::app::AppExit;
use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::input::ElementState;
use bevy::prelude::*;

pub fn exit_on_q_or_esc(mut key_events: EventReader<KeyboardInput>, mut out: EventWriter<AppExit>) {
    for event in key_events.iter() {
        if let Some(code) = event.key_code {
            if event.state == ElementState::Pressed
                && (code == KeyCode::Escape || code == KeyCode::Q)
            {
                out.send(AppExit);
            }
        }
    }
}
