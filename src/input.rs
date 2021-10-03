use bevy::app::AppExit;
use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::input::ElementState;
use bevy::prelude::*;
use bevy::render::camera::Camera;

use crate::TILE_SIZE;

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

pub struct WasdControl;

pub fn move_on_wasd(
    mut key_events: EventReader<KeyboardInput>,
    mut query: Query<&mut Transform, With<WasdControl>>,
) {
    let mut movement = Vec2::ZERO;
    for event in key_events.iter() {
        if let Some(code) = event.key_code {
            if event.state != ElementState::Pressed {
                continue;
            }
            let delta = match code {
                KeyCode::W => Vec2::new(0., 1.),
                KeyCode::A => Vec2::new(-1., 0.),
                KeyCode::S => Vec2::new(0., -1.),
                KeyCode::D => Vec2::new(1., 0.),
                _ => continue,
            };
            movement += delta * *TILE_SIZE;
        }
    }
    for mut transform in query.iter_mut() {
        transform.translation = (Vec2::from(transform.translation) + movement).extend(0.);
    }
}
