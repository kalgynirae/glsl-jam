use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::pipeline::{PipelineDescriptor, RenderPipeline};
use bevy::render::render_graph::{base, RenderGraph, RenderResourcesNode};
use bevy::render::renderer::RenderResources;
use bevy::render::shader::ShaderStages;

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(animate.system())
            .add_system(resize.system());
    }
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "c5408f9c-d13a-4788-91b0-85c61c81ca4b"]
pub struct TimeUniform {
    value: f32,
}

#[derive(Debug, RenderResources, Default, TypeUuid)]
#[uuid = "173b7369-9615-4039-a905-ab6bd3f51c37"]
pub struct WindowsizeUniform {
    value: Vec2,
}

pub fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_graph: ResMut<RenderGraph>,
    window: Res<WindowDescriptor>,
) {
    asset_server.watch_for_changes().unwrap();

    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: asset_server.load::<Shader, _>("shaders/bg.vert"),
        fragment: Some(asset_server.load::<Shader, _>("shaders/bg.frag")),
    }));

    render_graph.add_system_node(
        "time_uniform",
        RenderResourcesNode::<TimeUniform>::new(true),
    );
    render_graph.add_system_node(
        "windowsize_uniform",
        RenderResourcesNode::<WindowsizeUniform>::new(true),
    );

    render_graph
        .add_node_edge("time_uniform", base::node::MAIN_PASS)
        .unwrap();
    render_graph
        .add_node_edge("windowsize_uniform", base::node::MAIN_PASS)
        .unwrap();

    commands
        .spawn_bundle(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1., 1.)))),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_scale(Vec3::new(window.width, window.height, 1.)),
            ..Default::default()
        })
        .insert(TimeUniform { value: 0. })
        .insert(WindowsizeUniform {
            value: Vec2::new(window.width, window.height),
        });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn animate(time: Res<Time>, mut query: Query<&mut TimeUniform>) {
    let mut time_uniform = query.single_mut().unwrap();
    time_uniform.value = time.seconds_since_startup() as f32;
}

fn resize(windows: Res<Windows>, mut query: Query<(&mut Transform, &mut WindowsizeUniform)>) {
    if let Some(window) = windows.get_primary() {
        for (mut transform, mut windowsize) in query.iter_mut() {
            transform.scale = Vec3::new(
                window.physical_width() as f32,
                window.physical_height() as f32,
                1.,
            );
            windowsize.value = Vec2::new(
                window.physical_width() as f32,
                window.physical_height() as f32,
            );
        }
    }
}
