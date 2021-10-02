use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::pipeline::{PipelineDescriptor, RenderPipeline};
use bevy::render::render_graph::{base, AssetRenderResourcesNode, RenderGraph};
use bevy::render::renderer::RenderResources;
use bevy::render::shader::{ShaderStage, ShaderStages};
use bevy::window::WindowResized;

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<BgMaterial>()
            .add_startup_system(setup_background.system());
    }
}

pub struct Background;

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "c5408f9c-d13a-4788-91b0-85c61c81ca4b"]
pub struct BgMaterial {
    pub color: Color,
}

pub fn setup_background(
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
        "bg_material",
        AssetRenderResourcesNode::<BgMaterial>::new(true),
    );

    render_graph
        .add_node_edge("bg_material", base::node::MAIN_PASS)
        .unwrap();

    commands
        .spawn_bundle(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(5., 5.)))),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        })
        .insert(Background);
}
