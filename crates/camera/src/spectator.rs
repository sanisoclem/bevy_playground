use bevy::{
  input::{keyboard::KeyCode, mouse::MouseMotion, Input},
  prelude::*,
};
use std::f32::consts::FRAC_PI_2;

#[derive(Component)]
pub struct SpectatorCamera {
  speed: f32,
  sensitivity: f32,
  pitch: f32,
  yaw: f32,
}

impl Default for SpectatorCamera {
  fn default() -> Self {
    Self {
      speed: 100.0,
      sensitivity: 0.1,
      pitch: 0.0,
      yaw: 0.0,
    }
  }
}

pub struct SpectatorCameraPlugin;

impl Plugin for SpectatorCameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(setup)
      .add_system(camera_movement_system);
  }
}

pub fn setup(mut commands: Commands, mut window: ResMut<Windows>) {
  commands
    .spawn_bundle(PerspectiveCameraBundle {
      transform: Transform::from_xyz(0., 10.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..default()
    })
    .insert(SpectatorCamera::default());

  if let Some(window) = window.get_primary_mut() {
    window.set_cursor_visibility(false);
    window.set_cursor_lock_mode(true);
  }
}

fn forward_vector(rotation: &Quat) -> Vec3 {
  rotation.mul_vec3(Vec3::Z).normalize()
}

fn forward_walk_vector(rotation: &Quat) -> Vec3 {
  let f = forward_vector(rotation);
  let f_flattened = Vec3::new(f.x, 0.0, f.z).normalize();
  f_flattened
}

fn strafe_vector(rotation: &Quat) -> Vec3 {
  Quat::from_rotation_y(90.0f32.to_radians())
    .mul_vec3(forward_walk_vector(rotation))
    .normalize()
}

fn movement_axis(input: &Res<Input<KeyCode>>, plus: KeyCode, minus: KeyCode) -> f32 {
  let mut axis = 0.0;
  if input.pressed(plus) {
    axis += 1.0;
  }
  if input.pressed(minus) {
    axis -= 1.0;
  }
  axis
}

fn camera_movement_system(
  time: Res<Time>,
  keyboard_input: Res<Input<KeyCode>>,
  mut mouse_motion_event_reader: EventReader<MouseMotion>,
  mut query: Query<(&mut SpectatorCamera, &mut Transform)>,
) {
  let axis_x = movement_axis(&keyboard_input, KeyCode::D, KeyCode::A);
  let axis_z = movement_axis(&keyboard_input, KeyCode::S, KeyCode::W);
  let axis_y = movement_axis(&keyboard_input, KeyCode::Space, KeyCode::LShift);

  if let Ok((mut options, mut transform)) = query.get_single_mut() {
    let delta_time = time.delta_seconds();
    let delta_f = forward_walk_vector(&transform.rotation) * axis_z * options.speed * delta_time;
    let delta_strafe = strafe_vector(&transform.rotation) * axis_x * options.speed * delta_time;
    let delta_float = Vec3::Y * axis_y * options.speed * delta_time;

    transform.translation += delta_f + delta_strafe + delta_float;

    let mut delta = Vec2::ZERO;
    for mouse_move in mouse_motion_event_reader.iter() {
      delta += mouse_move.delta;
    }

    if delta == Vec2::ZERO {
      return;
    }
    options.pitch = (options.pitch + delta.y * options.sensitivity * delta_time)
      .clamp(-FRAC_PI_2 + 1.0, FRAC_PI_2);
    options.yaw = options.yaw - delta.x * options.sensitivity * delta_time;
    transform.rotation = Quat::from_axis_angle(Vec3::Y, options.yaw);
    transform.rotation *= Quat::from_axis_angle(-Vec3::X, options.pitch);
  }
}
