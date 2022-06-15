use super::VoxelId;
use bevy::{
  prelude::*,
  tasks::{AsyncComputeTaskPool, Task},
};
use block_mesh::{
  ndshape::{RuntimeShape, Shape},
  MergeVoxel, Voxel, VoxelVisibility,
};
use noise::{utils::*, *};

#[derive(Debug, PartialEq, Clone, Eq, Copy)]
pub enum VoxelType {
  Air,
  Dirt,
}
impl VoxelType {
  pub fn to_mat_id(&self) -> u8 {
    match self {
      VoxelType::Air => 0,
      VoxelType::Dirt => 1,
    }
  }
}
impl Voxel for VoxelType {
  fn get_visibility(&self) -> VoxelVisibility {
    if let VoxelType::Air = self {
      VoxelVisibility::Empty
    } else {
      VoxelVisibility::Opaque
    }
  }
}

impl MergeVoxel for VoxelType {
  type MergeValue = Self;

  fn merge_value(&self) -> Self::MergeValue {
    *self
  }
}

#[derive(Default)]
pub struct VoxelGenerator;

impl VoxelGenerator {
  pub fn load_voxel_data(
    &self,
    thread_pool: &Res<AsyncComputeTaskPool>,
    origin: VoxelId,
    shape: RuntimeShape<u32, 3>,
  ) -> Task<super::ChunkVoxelData> {
    thread_pool.spawn(async move {
      let bias = 0.0;
      let scale = [0.01, 0.01, 1.0];
      let bg = Perlin::new();
      // let bg_scaled = ScaleBias::new(&bg).set_bias(bias).set_scale(0.1);
      let ridged = RidgedMulti::new()
        .set_frequency(2.0)
        .set_lacunarity(2.20703125)
        .set_octaves(3);
      let ridged_scaled = ScalePoint::new(&ridged).set_all_scales(0.5, 0.5, 1.0, 1.0);
      let fbm = Fbm::new();
      let t = Billow::new();

      let baseContinentDef_fb0 = Fbm::new()
        .set_frequency(1.0)
        .set_persistence(0.5)
        .set_lacunarity(2.208984375)
        .set_octaves(14);

      let baseContinentDef_cu = Curve::new(&baseContinentDef_fb0)
        .add_control_point(-2.0, -2.0)
        .add_control_point(-1.0, -1.0)
        .add_control_point(0.0, 0.0)
        .add_control_point(0.5, 0.01)
        .add_control_point(1.0, 0.02)
        .add_control_point(2.0, 0.03);

      let scaled_conti = ScalePoint::new(&baseContinentDef_cu).set_all_scales(0.1, 0.1, 1.0, 1.0);
      let generator =
        ScalePoint::new(&scaled_conti).set_all_scales(scale[0], scale[1], scale[2], 1.0);

      let mut buffer = Vec::with_capacity(shape.usize());
      for i in 0..shape.size() {
        let [x, y, z] = shape.delinearize(i);
        let height = generator.get([x as f64 + origin.x() as f64, z as f64 + origin.z() as f64]);
        let sdf = y as f32 - ((height + 1.0) * 25.) as f32;
        buffer.push(sdf);
      }
      super::ChunkVoxelData { voxels: buffer }
    })
  }
}
