use bevy::prelude::*;

mod background;
mod input;

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let tile_size = Vec2::splat(24.);
    let map_size = Vec2::splat(480.);

    let position = Vec2::new(1., 1.);

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(assets.load("sprites/red.png").into()),
        ..Default::default()
    });
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "ld(7*7)".to_string(),
            width: 480.,
            height: 480.,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(background::BackgroundPlugin)
        .add_startup_system(setup.system())
        .add_system(input::exit_on_q_or_esc.system())
        .add_system(input::move_camera.system())
        .run();
}
