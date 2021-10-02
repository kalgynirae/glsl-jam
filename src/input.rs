use bevy::app::AppExit;
use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::input::ElementState;
use bevy::prelude::*;
use bevy::render::camera::Camera;

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

pub fn move_camera(
    mut key_events: EventReader<KeyboardInput>,
    mut out: EventWriter<AppExit>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    let mut movement = Vec2::ZERO;
    for event in key_events.iter() {
        if let Some(code) = event.key_code {
            if event.state != ElementState::Pressed {
                continue;
            }
            let delta = match code {
                KeyCode::W => Vec2::new(0., -5.),
                KeyCode::A => Vec2::new(5., 0.),
                KeyCode::S => Vec2::new(0., 5.),
                KeyCode::D => Vec2::new(-5., 0.),
                _ => continue,
            };
            movement += delta;
        }
    }
    for mut transform in cameras.iter_mut() {
        transform.translation = (Vec2::from(transform.translation) + movement).extend(0.);
    }
}
