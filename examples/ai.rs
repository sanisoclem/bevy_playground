use bevy::prelude::*;

const ARENA_SIZE: f32 = 1000.;
const WALL_HEIGHT: f32 = 50.;
const WALL_THICKNESS: f32 = 10.;

fn main() {
  App::new()
    .insert_resource(WindowDescriptor {
      title: "AI".to_string(),
      width: 1920.,
      height: 1080.,
      ..Default::default()
    })
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_plugin(debug::DebugUIPlugin)
    .add_plugin(camera::SpectatorCameraPlugin)
    .add_startup_system(setup)
    .run();
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // SOUTH WALL
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Box::new(ARENA_SIZE, WALL_HEIGHT, WALL_THICKNESS))),
    material: materials.add(StandardMaterial {
      base_color: Color::LIME_GREEN,
      ..default()
    }),
    transform: Transform::from_xyz(0.0, WALL_HEIGHT/2., (ARENA_SIZE/2.) - (WALL_THICKNESS/2.)),
    ..default()
  });
  // NORTH WALL
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Box::new(ARENA_SIZE, WALL_HEIGHT, WALL_THICKNESS))),
    material: materials.add(StandardMaterial {
      base_color: Color::LIME_GREEN,
      ..default()
    }),
    transform: Transform::from_xyz(0.0, WALL_HEIGHT/2., (ARENA_SIZE/-2.) + (WALL_THICKNESS/2.)),
    ..default()
  });
  // EAST WALL
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Box::new(WALL_THICKNESS, WALL_HEIGHT, ARENA_SIZE - WALL_THICKNESS * 2.))),
    material: materials.add(StandardMaterial {
      base_color: Color::AQUAMARINE,
      ..default()
    }),
    transform: Transform::from_xyz((ARENA_SIZE/2.) - (WALL_THICKNESS/2.), WALL_HEIGHT/2., 0.),
    ..default()
  });
  // WEST WALL
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Box::new(WALL_THICKNESS, WALL_HEIGHT, ARENA_SIZE - WALL_THICKNESS * 2.))),
    material: materials.add(StandardMaterial {
      base_color: Color::AQUAMARINE,
      ..default()
    }),
    transform: Transform::from_xyz((ARENA_SIZE/-2.) + (WALL_THICKNESS/2.), WALL_HEIGHT/2., 0.),
    ..default()
  });
  // ARENA GROUND
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Plane { size: ARENA_SIZE })),
    material: materials.add(StandardMaterial {
      base_color: Color::NAVY,
      ..default()
    }),
    transform: Transform::from_xyz(0.0, 0.0, 0.0),
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
