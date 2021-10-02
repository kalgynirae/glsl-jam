use bevy::prelude::*;
use lazy_static::lazy_static;

mod background;
mod input;

lazy_static!{
    pub static ref TILE_SIZE: Vec2 = Vec2::splat(24.);
    pub static ref MAP_SIZE: Vec2 = *TILE_SIZE * 16.;
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(assets.load("sprites/red.png").into()),
        ..Default::default()
    }).insert(input::WasdControl);
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "ld(7*7)".to_string(),
            width: MAP_SIZE.x,
            height: MAP_SIZE.y,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(background::BackgroundPlugin)
        .add_startup_system(setup.system())
        .add_system(input::exit_on_q_or_esc.system())
        .add_system(input::move_on_wasd.system())
        .run();
}
