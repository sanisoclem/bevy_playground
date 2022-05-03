use bevy::prelude::*;
use voxel_terrain::{ChunkSpawner, VoxelTerrainPlugin};

fn main() {
  App::new()
    .insert_resource(WindowDescriptor {
      title: "Voxel Terrain".to_string(),
      width: 1920.,
      height: 1080.,
      ..Default::default()
    })
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_plugin(debug::DebugUIPlugin)
    .add_plugin(VoxelTerrainPlugin)
    //.add_plugin(camera::RtsCameraPlugin)
    .add_plugin(camera::SpectatorCameraPlugin)
    .add_startup_system(setup)
    .add_system(add_chunk_spawner)
    .run();
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // cube
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 4.0 })),
    material: materials.add(StandardMaterial {
      base_color: Color::LIME_GREEN,
      ..default()
    }),
    transform: Transform::from_xyz(0.0, 20.5, 0.0),
    ..default()
  });
  // light
  commands.spawn_bundle(PointLightBundle {
    point_light: PointLight {
      intensity: 1500.0,
      shadows_enabled: true,
      ..default()
    },
    transform: Transform::from_xyz(4.0, 20.0, 4.0),
    ..default()
  });

  // ambient light
  commands.insert_resource(AmbientLight {
    color: Color::ORANGE_RED,
    brightness: 0.02,
  });

  // directional 'sun' light
  const HALF_SIZE: f32 = 10.0;
  commands.spawn_bundle(DirectionalLightBundle {
    directional_light: DirectionalLight {
      shadow_projection: OrthographicProjection {
        left: -HALF_SIZE,
        right: HALF_SIZE,
        bottom: -HALF_SIZE,
        top: HALF_SIZE,
        near: -10.0 * HALF_SIZE,
        far: 10.0 * HALF_SIZE,
        ..default()
      },
      shadows_enabled: true,
      ..default()
    },
    transform: Transform {
      translation: Vec3::new(0.0, 2.0, 0.0),
      rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
      ..default()
    },
    ..default()
  });
}

fn add_chunk_spawner(
  mut commands: Commands,
  qry: Query<Entity, (With<camera::SpectatorCamera>, Without<ChunkSpawner>)>,
) {
  for entity in qry.iter() {
    commands.entity(entity).insert(ChunkSpawner::default());
  }
}
