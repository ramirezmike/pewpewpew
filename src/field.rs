use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::ShaderStages,
    },
};
use crate::{asset_loader, player, };

static SCALE:f32 = 30.0;
static SPEED:f32 = 0.005;

pub struct LevelReady(pub bool);
pub struct FieldPlugin;
impl Plugin for FieldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(LevelReady(false))
            .init_resource::<GameMeshes>()
            .add_asset::<FieldMaterial>()
            .add_system_set(
               SystemSet::on_enter(crate::AppState::Loading)
                         .with_system(load_assets.system())
            )
            .add_system_set(
                SystemSet::on_enter(crate::AppState::InGame)
                    .with_system(load_level.system().label("loading_level"))
                    .with_system(crate::camera::create_camera.system().after("loading_level"))
                    .with_system(set_clear_color.system().after("loading_level"))

            )
            .add_system_set(
                SystemSet::on_exit(crate::AppState::InGame)
                    .with_system(cleanup_environment.system())
            )
            .add_system_set(
               SystemSet::on_update(crate::AppState::InGame)
                    .with_system(player::player_input.system())
                    .with_system(animate_shader.system())
                    .with_system(player::update_moveables.system())
            );
    }
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c620"]
struct FieldMaterial {
    pub color: Color,
}
#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4b8a-d555-4fc2-ba9f-4c880063ba92"]
struct FieldShaderSettings {
    time: f32,
    speed: f32,
    scale: f32
}

#[derive(Default)]
pub struct GameMeshes {
    pub field: Handle<Mesh>,
    pub field_material: Handle<StandardMaterial>,
    pub field_pipeline: Handle<PipelineDescriptor>,
}

fn load_assets(
//    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_meshes: ResMut<GameMeshes>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut render_graph: ResMut<RenderGraph>,
    mut loading: ResMut<asset_loader::AssetsLoading>,
) {
    asset_server.watch_for_changes().unwrap();

    //game_meshes.field = asset_server.load("models/meshes.glb#Mesh0/Primitive0");
    //loading.asset_handles.push(game_meshes.field.clone_untyped());

//  let texture_handle = asset_server.load("models/field.png");
//  game_meshes.field_material = materials.add(StandardMaterial {
//      base_color_texture: Some(texture_handle.clone()),
//      unlit: true,
//      ..Default::default()
//  });

    let vertex = asset_server.load::<Shader, _>("shaders/hot.vert");
    let fragment = asset_server.load::<Shader, _>("shaders/hot.frag");
    loading.asset_handles.push(vertex.clone_untyped());
    loading.asset_handles.push(fragment.clone_untyped());
    game_meshes.field_pipeline = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex, 
        fragment: Some(fragment),
    }));


    render_graph.add_system_node(
        "field_material",
        AssetRenderResourcesNode::<FieldMaterial>::new(true),
    );
    render_graph.add_system_node(
        "shader_settings",
        RenderResourcesNode::<FieldShaderSettings>::new(true),
    );

    render_graph
        .add_node_edge("field_material", base::node::MAIN_PASS)
        .unwrap();
}

fn cleanup_environment(
) {
}

struct Field;

fn load_level( 
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FieldMaterial>>,
    mut level_ready: ResMut<LevelReady>,
    game_meshes: Res<GameMeshes>,
) {
    let mut transform = Transform::identity();
    transform.apply_non_uniform_scale(Vec3::new(SCALE, 1.0, SCALE)); 

    let material = materials.add(FieldMaterial {
        color: Color::rgb(0.0, 0.8, 0.0),
    });

    commands.spawn_bundle(PbrBundle {
                transform,
                mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
                render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                    game_meshes.field_pipeline.clone(),
                )]),
                ..Default::default()
            })
            .insert(Field {})
            .insert(FieldShaderSettings { time: 0.0, speed: SPEED, scale: SCALE })
            .insert(material);

    player::spawn_player(&mut commands, &mut meshes);

    level_ready.0 = true;
}

fn animate_shader(time: Res<Time>, mut query: Query<&mut FieldShaderSettings>) {
    let shader_settings = query.single_mut();
    shader_settings.unwrap().time = time.seconds_since_startup() as f32;
}

fn set_clear_color(
    mut clear_color: ResMut<ClearColor>,
) {
    clear_color.0 = Color::hex("21123d").unwrap();
}
