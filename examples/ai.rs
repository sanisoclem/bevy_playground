use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::data::arena};

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
    .run();
}
fn spawn_critters(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let rad = 0.4;
  let origin = Vec3::new(0.0, 15.0, 0.0);

  commands
    .spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Icosphere {
        radius: rad,
        ..default()
      })),
      material: materials.add(StandardMaterial {
        base_color: Color::LIME_GREEN,
        ..default()
      }),
      transform: Transform::from_xyz(origin.x, 1.0, 1.0),
      ..default()
    })
    .insert(RigidBody::Dynamic)
    .insert(Collider::ball(rad))
    .insert(Restitution::coefficient(0.7));

  let head = commands
    .spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Icosphere {
        radius: rad,
        ..default()
      })),
      material: materials.add(StandardMaterial {
        base_color: Color::GRAY,
        ..default()
      }),
      transform: Transform::from_xyz(origin.x, origin.y, origin.z),
      ..default()
    })
    .insert(RigidBody::Dynamic)
    .insert(Collider::ball(rad))
    .insert(Restitution::coefficient(0.7))
    .id();
  let joint = SphericalJointBuilder::new().local_anchor2(Vec3::new(0.0, 0.0, rad * 2.));
  let body = commands
    .spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Icosphere {
        radius: rad,
        ..default()
      })),
      material: materials.add(StandardMaterial {
        base_color: Color::GRAY,
        ..default()
      }),
      transform: Transform::from_xyz(origin.x, origin.y, origin.z + rad * 2.),
      ..default()
    })
    .insert(RigidBody::Dynamic)
    .insert(Collider::ball(rad))
    .insert(Restitution::coefficient(0.7))
    .with_children(|children| {
      children.spawn().insert(ImpulseJoint::new(head, joint));
    })
    .id();
  let joint2 = SphericalJointBuilder::new().local_anchor2(Vec3::new(0.0, 0.0, rad * 2.));
  let tail = commands
    .spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Icosphere {
        radius: rad,
        ..default()
      })),
      material: materials.add(StandardMaterial {
        base_color: Color::GRAY,
        ..default()
      }),
      transform: Transform::from_xyz(origin.x, origin.y, origin.z + rad * 4.),
      ..default()
    })
    .insert(RigidBody::Dynamic)
    .insert(Collider::ball(rad))
    .insert(Restitution::coefficient(0.7))
    .with_children(|children| {
      children.spawn().insert(ImpulseJoint::new(body, joint2));
    })
    .id();
}

// fn create_ball_joints(
//   mut commands: Commands,
//   mut meshes: ResMut<Assets<Mesh>>,
//   mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//   let num = 15;
//   let rad = 0.4;
//   let shift = 1.0;

//   let mut body_entities = Vec::new();

//   for k in 0..num {
//     for i in 0..num {
//       let fk = k as f32;
//       let fi = i as f32;

//       let rigid_body = if i == 0 && (k % 4 == 0 || k == num - 1) {
//         RigidBody::Fixed
//       } else {
//         RigidBody::Dynamic
//       };

//       let child_entity = commands
//         .spawn_bundle(PbrBundle {
//           mesh: meshes.add(Mesh::from(shape::Icosphere {
//             radius: rad,
//             ..default()
//         })),
//           material: materials.add(StandardMaterial {
//             base_color: Color::GRAY,
//             ..default()
//           }),
//           transform: Transform::from_xyz(
//             fk * shift,
//             10.0,
//             fi * shift,
//           ),
//           ..default()
//         })
//         .insert(rigid_body)
//         .insert(Collider::ball(rad))
//         .id();

//       // Vertical joint.
//       if i > 0 {
//         let parent_entity = *body_entities.last().unwrap();
//         let joint = SphericalJointBuilder::new().local_anchor2(Vec3::new(0.0, 0.0, -shift));
//         commands.entity(child_entity).with_children(|children| {
//           // NOTE: we want to attach multiple impulse joints to this entity, so
//           //       we need to add the components to children of the entity. Otherwise
//           //       the second joint component would just overwrite the first one.
//           children
//             .spawn()
//             .insert(ImpulseJoint::new(parent_entity, joint));
//         });
//       }

//       // Horizontal joint.
//       if k > 0 {
//         let parent_index = body_entities.len() - num;
//         let parent_entity = body_entities[parent_index];
//         let joint = SphericalJointBuilder::new().local_anchor2(Vec3::new(-shift, 0.0, 0.0));
//         commands.entity(child_entity).with_children(|children| {
//           // NOTE: we want to attach multiple impulse joints to this entity, so
//           //       we need to add the components to children of the entity. Otherwise
//           //       the second joint component would just overwrite the first one.
//           children
//             .spawn()
//             .insert(ImpulseJoint::new(parent_entity, joint));
//         });
//       }

//       body_entities.push(child_entity);
//     }
//   }
// }

fn spawn_arena(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let arena_color = Color::rgb(0.3, 0.5, 0.3);
  // ARENA GROUND
  commands
    .spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Plane { size: ARENA_SIZE })),
      material: materials.add(StandardMaterial {
        base_color: arena_color.clone(),
        ..default()
      }),
      transform: Transform::from_xyz(0.0, 0.0, 0.0),
      ..default()
    })
    .insert(Collider::cuboid(ARENA_SIZE / 2., 0.1, ARENA_SIZE / 2.));

  // SOUTH WALL
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Box::new(
      ARENA_SIZE,
      WALL_HEIGHT,
      WALL_THICKNESS,
    ))),
    material: materials.add(StandardMaterial {
      base_color: arena_color.clone(),
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
      base_color: arena_color.clone(),
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
  // light
  commands.spawn_bundle(PointLightBundle {
    transform: Transform::from_xyz(4.0, 8.0, 4.0),
    ..default()
  });
}
