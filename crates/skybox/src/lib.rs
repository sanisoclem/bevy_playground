use bevy::prelude::*;

mod image;
mod material;

#[derive(Component)]
pub struct SkyboxCamera;

#[derive(Component)]
pub struct SkyboxBox;

#[derive(Default)]
pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugin(MaterialPlugin::<material::SkyMaterial>::default())
  }
}

pub fn create_skybox(
  mut commands: Commands,
  pipelines: ResMut<Assets<PipelineDescriptor>>,
  shaders: ResMut<Assets<Shader>>,
  render_graph: ResMut<RenderGraph>,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<crate::material::SkyMaterial>>,
  mut meshes: ResMut<Assets<Mesh>>,
) {
  if let Some(image) = &plugin.image {
    let mesh = get_mesh(image).expect("Good image");
    // Load image as a texture asset.
    let texture_handle: Handle<Texture> = asset_server.load(image.as_str());

    // Even before the texture is loaded we can updated the material.
    let sky_material = materials.add(SkyMaterial {
      texture: texture_handle,
    });

    let render_pipelines = SkyMaterial::pipeline(pipelines, shaders, render_graph);

    // Create the PbrBundle tagged as a skybox.
    commands
      .spawn()
      .insert_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        render_pipelines: render_pipelines.clone(),
        ..Default::default()
      })
      .insert(sky_material)
      .insert(crate::SkyboxBox);
  }
}

fn create_pipeline(
  mut commands: Commands,
  camera_query: Query<(Entity, &SkyboxCamera)>,
  skybox_query: Query<(Entity, &SkyboxBox)>,
  mut active_cameras: ResMut<bevy::render::camera::ActiveCameras>,
  plugin: Res<crate::SkyboxPlugin>,
) {
  // If more than one SkyboxCamera is defined then only one is used.
  if let Some((cam, _)) = camera_query.iter().next() {
    // Add a secondary camera as a child of the main camera
    let child_entity = commands
      .spawn()
      .insert_bundle(PerspectiveCameraBundle::default())
      .id();
    commands.entity(cam).push_children(&[child_entity]);

    // Make the secondary camera active.
    active_cameras.add(&plugin.camera_name);

    // Assign the skybox to the secondary camera.
    for s in skybox_query.iter() {
      active_cameras
        .get_mut(&plugin.camera_name)
        .expect("Camera defined")
        .entity = Some(s.0);
    }
  }
}
