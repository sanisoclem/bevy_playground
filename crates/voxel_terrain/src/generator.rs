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
      let perlin = Perlin::new();
      let ridged = RidgedMulti::new()
        .set_frequency(2.0)
        .set_lacunarity(2.20703125)
        .set_octaves(3);
      let fbm = Fbm::new();
      let blend = Blend::new(&perlin, &ridged, &fbm);
      let scale_bias = ScaleBias::new(&blend).set_bias(bias);
      let generator =
        ScalePoint::new(&scale_bias).set_all_scales(scale[0], scale[1], scale[2], 1.0);

      let mut buffer = Vec::with_capacity(shape.usize());
      for i in 0..shape.size() {
        let [x, y, z] = shape.delinearize(i);
        let height = generator.get([x as f64 + origin.x() as f64, z as f64 + origin.z() as f64]);
        let sdf = y as f32 - ((height + 1.0) * 25. + 1.0) as f32;
        buffer.push(sdf);
      }
      super::ChunkVoxelData { voxels: buffer }
    })
  }
}