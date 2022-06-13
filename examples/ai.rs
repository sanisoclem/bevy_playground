use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_startup_system(spawn_critters)
    .add_startup_system(spawn_arena)
    .add_startup_system(create_ball_joints)
    .run();
}
fn spawn_critters(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands
    .spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Box::new(0.2, 0.2, 1.))),
      material: materials.add(StandardMaterial {
        base_color: Color::LIME_GREEN,
        ..default()
      }),
      transform: Transform::from_xyz(0.0, 50.0, 0.0),
      ..default()
    })
    .insert(RigidBody::Dynamic)
    .insert(Collider::ball(1.0))
    .insert(Restitution::coefficient(0.7));
}


fn create_ball_joints(mut commands: Commands) {
  let num = 15;
  let rad = 0.4;
  let shift = 1.0;

  let mut body_entities = Vec::new();

  for k in 0..num {
      for i in 0..num {
          let fk = k as f32;
          let fi = i as f32;

          let rigid_body = if i == 0 && (k % 4 == 0 || k == num - 1) {
              RigidBody::Fixed
          } else {
              RigidBody::Dynamic
          };

          let child_entity = commands
              .spawn_bundle(TransformBundle::from(Transform::from_xyz(
                  fk * shift,
                  0.0,
                  fi * shift,
              )))
              .insert(rigid_body)
              .insert(Collider::ball(rad))
              .id();

          // Vertical joint.
          if i > 0 {
              let parent_entity = *body_entities.last().unwrap();
              let joint = SphericalJointBuilder::new().local_anchor2(Vec3::new(0.0, 0.0, -shift));
              commands.entity(child_entity).with_children(|children| {
                  // NOTE: we want to attach multiple impulse joints to this entity, so
                  //       we need to add the components to children of the entity. Otherwise
                  //       the second joint component would just overwrite the first one.
                  children
                      .spawn()
                      .insert(ImpulseJoint::new(parent_entity, joint));
              });
          }

          // Horizontal joint.
          if k > 0 {
              let parent_index = body_entities.len() - num;
              let parent_entity = body_entities[parent_index];
              let joint = SphericalJointBuilder::new().local_anchor2(Vec3::new(-shift, 0.0, 0.0));
              commands.entity(child_entity).with_children(|children| {
                  // NOTE: we want to attach multiple impulse joints to this entity, so
                  //       we need to add the components to children of the entity. Otherwise
                  //       the second joint component would just overwrite the first one.
                  children
                      .spawn()
                      .insert(ImpulseJoint::new(parent_entity, joint));
              });
          }

          body_entities.push(child_entity);
      }
  }
}

fn spawn_arena(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // SOUTH WALL
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Box::new(
      ARENA_SIZE,
      WALL_HEIGHT,
      WALL_THICKNESS,
    ))),
    material: materials.add(StandardMaterial {
      base_color: Color::LIME_GREEN,
      ..default()
    }),
    transform: Transform::from_xyz(
      0.0,
      WALL_HEIGHT / 2.,
      (ARENA_SIZE / 2.) - (WALL_THICKNESS / 2.),
    ),
    ..default()
  });
  // NORTH WALL
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Box::new(
      ARENA_SIZE,
      WALL_HEIGHT,
      WALL_THICKNESS,
    ))),
    material: materials.add(StandardMaterial {
      base_color: Color::LIME_GREEN,
      ..default()
    }),
    transform: Transform::from_xyz(
      0.0,
      WALL_HEIGHT / 2.,
      (ARENA_SIZE / -2.) + (WALL_THICKNESS / 2.),
    ),
    ..default()
  });
  // EAST WALL
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Box::new(
      WALL_THICKNESS,
      WALL_HEIGHT,
      ARENA_SIZE - WALL_THICKNESS * 2.,
    ))),
    material: materials.add(StandardMaterial {
      base_color: Color::AQUAMARINE,
      ..default()
    }),
    transform: Transform::from_xyz(
      (ARENA_SIZE / 2.) - (WALL_THICKNESS / 2.),
      WALL_HEIGHT / 2.,
      0.,
    ),
    ..default()
  });
  // WEST WALL
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Box::new(
      WALL_THICKNESS,
      WALL_HEIGHT,
      ARENA_SIZE - WALL_THICKNESS * 2.,
    ))),
    material: materials.add(StandardMaterial {
      base_color: Color::AQUAMARINE,
      ..default()
    }),
    transform: Transform::from_xyz(
      (ARENA_SIZE / -2.) + (WALL_THICKNESS / 2.),
      WALL_HEIGHT / 2.,
      0.,
    ),
    ..default()
  });
  // ARENA GROUND
  commands
    .spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Plane { size: ARENA_SIZE })),
      material: materials.add(StandardMaterial {
        base_color: Color::NAVY,
        ..default()
      }),
      transform: Transform::from_xyz(0.0, 0.0, 0.0),
      ..default()
    })
    .insert(Collider::cuboid(ARENA_SIZE / 2., 0.1, ARENA_SIZE / 2.));
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
