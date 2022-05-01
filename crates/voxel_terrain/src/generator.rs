use super::VoxelId;
use bevy::{
  prelude::*,
  tasks::{AsyncComputeTaskPool, Task},
};
use block_mesh::{
  ndshape::{RuntimeShape, Shape},
  MergeVoxel, Voxel, VoxelVisibility,
};

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
    _origin: VoxelId,
    shape: RuntimeShape<u32, 3>,
  ) -> Task<super::ChunkVoxelData> {
    thread_pool.spawn(async move {
      let mut buffer = Vec::with_capacity(shape.usize());
      for i in 0..shape.size() {
        let [x, y, z] = shape.delinearize(i);
        buffer.push(if y > 1 {
          VoxelType::Air
        } else {
          VoxelType::Dirt
        });
      }
      super::ChunkVoxelData { voxels: buffer }
    })
  }
}
