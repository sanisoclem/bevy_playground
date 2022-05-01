use bevy::{math::Mat2, prelude::*};
use block_mesh::ndshape::RuntimeShape;
use lazy_static::*;
use std::{
  hash::Hash,
  ops::{Add, Sub},
};

lazy_static! {
  static ref ROTATE_4X: [Mat2; 4] = [
    Mat2::from_cols_array(&[0.0, 1.0, -1.0, 0.0]),
    Mat2::from_cols_array(&[-1.0, 0.0, 0.0, -1.0]),
    Mat2::from_cols_array(&[0.0, -1.0, 1.0, 0.0]),
    Mat2::from_cols_array(&[1.0, 0.0, 0.0, 1.0])
  ];
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default, Eq, Hash)]
pub struct ChunkId(i32, i32);
impl ChunkId {
  pub fn new(x: i32, y: i32) -> Self {
    Self(x, y)
  }

  #[inline]
  pub fn x(&self) -> i32 {
    self.0
  }

  #[inline]
  pub fn y(&self) -> i32 {
    self.1
  }
}
impl Add for ChunkId {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    Self(self.x() + other.x(), self.y() + other.y())
  }
}
impl Sub for ChunkId {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    Self(self.x() - other.x(), self.y() - other.y())
  }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default, Eq, Hash)]
pub struct VoxelId(i32, i32, i32);
impl VoxelId {
  #[inline]
  pub fn x(&self) -> i32 {
    self.0
  }

  #[inline]
  pub fn y(&self) -> i32 {
    self.1
  }

  #[inline]
  pub fn z(&self) -> i32 {
    self.2
  }
}
impl Add for VoxelId {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    Self(
      self.x() + other.x(),
      self.y() + other.y(),
      self.z() + other.z(),
    )
  }
}
impl Sub for VoxelId {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    Self(
      self.x() - other.x(),
      self.y() - other.y(),
      self.z() - other.z(),
    )
  }
}

pub struct CubicVoxelLayout {
  pub origin: ChunkId,
  voxel_side_length: f32,
  chunk_voxel_length: u32,
  chunk_voxel_height: u32,
  pub shape: RuntimeShape<u32, 3>,
}

impl CubicVoxelLayout {
  #[inline]
  pub fn chunk_side_length(&self) -> f32 {
    self.chunk_voxel_full_length() as f32 * self.voxel_side_length
  }

  #[inline]
  pub fn chunk_voxel_full_length(&self) -> u32 {
    1 + (self.chunk_voxel_length * 2)
  }

  #[inline]
  pub fn chunk_voxel_height(&self) -> u32 {
    self.chunk_voxel_height
  }

  #[inline]
  pub fn get_center_voxel(&self, chunk: &ChunkId) -> VoxelId {
    VoxelId(
      chunk.x() * self.chunk_voxel_full_length() as i32,
      0,
      chunk.y() * self.chunk_voxel_full_length() as i32,
    )
  }

  #[inline]
  pub fn get_origin(&self, chunk: &ChunkId) -> VoxelId {
    VoxelId(
      (chunk.x() * self.chunk_voxel_full_length() as i32) - self.chunk_voxel_length as i32,
      0,
      (chunk.y() * self.chunk_voxel_full_length() as i32) - self.chunk_voxel_length as i32,
    )
  }

  #[inline]
  pub fn get_voxel(&self, chunk: &ChunkId, x: i32, y: i32, z: i32) -> VoxelId {
    let vx = x + (chunk.x() * self.chunk_voxel_full_length() as i32);
    let vz = z + (chunk.y() * self.chunk_voxel_full_length() as i32);
    VoxelId(vx, y, vz)
  }

  pub fn new(
    origin: ChunkId,
    voxel_side_length: f32,
    chunk_voxel_length: u32,
    chunk_voxel_height: u32,
  ) -> Self {
    let side_length = 1 + (chunk_voxel_length * 2);
    Self {
      origin,
      voxel_side_length,
      chunk_voxel_length,
      chunk_voxel_height,
      shape: RuntimeShape::<u32, 3>::new([
        side_length + 2,
        chunk_voxel_height + 2,
        side_length + 2,
      ]),
    }
  }

  pub fn get_chunk_neighbors(&self, chunk: &ChunkId, distance: i32) -> Vec<ChunkId> {
    (1..=distance)
      .flat_map(move |ring| {
        (0..(2 * ring)).flat_map(move |offset| {
          ROTATE_4X
            .iter()
            .map(move |rot| rot.mul_vec2(Vec2::new((-ring + offset) as f32, -ring as f32)))
            .map(move |v2| *chunk + ChunkId::new(v2.x as i32, v2.y as i32))
        })
      })
      .collect()
  }

  pub fn get_chunk_voxels(&self, chunk: &ChunkId) -> Vec<VoxelId> {
    (0..self.chunk_voxel_full_length())
      .flat_map(|x| {
        (0..self.chunk_voxel_full_length()).flat_map(move |z| {
          (0..self.chunk_voxel_height).map(move |y| {
            self.get_voxel(
              chunk,
              x as i32 - self.chunk_voxel_length as i32,
              y as i32,
              z as i32 - self.chunk_voxel_length as i32,
            )
          })
        })
      })
      .collect()
  }

  pub fn chunk_to_space(&self, chunk: &ChunkId) -> Vec3 {
    self.voxel_to_space(&self.get_center_voxel(chunk))
  }

  pub fn voxel_to_chunk(&self, voxel: &VoxelId) -> ChunkId {
    let x = (voxel.x() + self.chunk_voxel_length as i32)
      .div_euclid(self.chunk_voxel_full_length() as i32);
    let y = (voxel.z() + self.chunk_voxel_length as i32)
      .div_euclid(self.chunk_voxel_full_length() as i32);
    ChunkId::new(x, y)
  }

  pub fn voxel_to_space(&self, voxel: &VoxelId) -> Vec3 {
    let center = self.get_center_voxel(&self.origin);
    let transposed = *voxel - center;
    let x = transposed.x() as f32 * self.voxel_side_length;
    let y = transposed.y() as f32 * self.voxel_side_length;
    let z = transposed.z() as f32 * self.voxel_side_length;
    Vec3::new(x, y, z)
  }

  pub fn space_to_voxel(&self, space: &Vec3) -> VoxelId {
    let center = self.get_center_voxel(&self.origin);
    let divisor = self.voxel_side_length as i32;
    let x = (space.x as i32).div_euclid(divisor);
    let y = (space.y as i32).div_euclid(self.chunk_voxel_height as i32);
    let z = (space.z as i32).div_euclid(divisor);
    VoxelId(x, y, z) + center
  }

  pub fn space_to_chunk(&self, space: &Vec3) -> ChunkId {
    self.voxel_to_chunk(&self.space_to_voxel(space))
  }

  pub fn get_chunk_distance(&self, a: &ChunkId, b: &ChunkId) -> f32 {
    (self.chunk_to_space(a) - self.chunk_to_space(b)).length()
  }
}
impl Default for CubicVoxelLayout {
  fn default() -> Self {
    Self::new(ChunkId::default(), 1.0, 50, 20)
  }
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;
  use proptest::prelude::*;

  proptest! {
      #[test]
      fn chunk_should_have_appropriate_number_of_neighbors(x1 in -10000i32..=10000, y1 in -10000i32..=10000, x2 in -10000i32..=10000, z2 in -10000i32..=10000, voxel_length in 1u32..50, distance in 1i32..10) {
          let layout = CubicVoxelLayout::new(ChunkId(x1, y1), 1.0, voxel_length, voxel_length);
          let voxel = VoxelId(x2, 0, z2);
          let chunk = layout.voxel_to_chunk(&voxel);
          let count =  layout.get_chunk_neighbors(&chunk, distance).len();
          let expected = ((distance * 2) + 1) * ((distance * 2) + 1) - 1;
          assert_eq!(expected, count as i32);
      }

      #[test]
      fn neighbor_should_have_correct_distance(x1 in -10000i32..=10000, y1 in -10000i32..=10000, x2 in -10000i32..=10000, z2 in -10000i32..=10000, voxel_length in 1u32..50, distance in 1i32..10) {
          let layout = CubicVoxelLayout::new(ChunkId(x1, y1), 1.0, voxel_length, voxel_length);
          let voxel = VoxelId(x2, 0, z2);
          let chunk = layout.voxel_to_chunk(&voxel);
          for neighbor in layout.get_chunk_neighbors(&chunk, distance) {
              let diff = neighbor - chunk;
              let x = diff.x().abs();
              let y = diff.y().abs();
              let max = if x > y { x } else { y };
              assert!(max <= distance);
          }
      }

      #[test]
      fn neighbor_should_be_mutual(x1 in -10000i32..=10000, y1 in -10000i32..=10000, x2 in -10000i32..=10000, z2 in -10000i32..=10000, voxel_length in 1u32..50, distance in 1i32..10) {
          let layout = CubicVoxelLayout::new(ChunkId(x1, y1), 1.0, voxel_length, voxel_length);
          let voxel = VoxelId(x2, 0, z2);
          let chunk = layout.voxel_to_chunk(&voxel);
          for neighbor in layout.get_chunk_neighbors(&chunk, distance) {
              let ns: Vec<_> = layout.get_chunk_neighbors(&neighbor, distance);
              let original: Vec<_> = ns.clone().into_iter().filter(|n| *n == chunk).collect();
              assert_eq!(original.len(), 1);
              assert_eq!(original[0], chunk);
          }
      }

      #[test]
      fn chunk_space_coordinates_should_be_zero_when_at_origin(x1 in -10000i32..=10000, y1 in -10000i32..=10000, voxel_length in 1u32..50) {
          let layout = CubicVoxelLayout::new(ChunkId(x1, y1), 1.0, voxel_length, voxel_length);
          let coords = layout.chunk_to_space(&layout.origin);
          assert_eq!(coords.x, 0.0);
          assert_eq!(coords.y, 0.0);
          assert_eq!(coords.z, 0.0);
      }

      #[test]
      fn voxel_space_coordinates_should_be_reversible(x1 in -10000i32..=10000, y1 in -10000i32..=10000, x2 in -10000i32..=10000, z2 in -10000i32..=10000, voxel_length in 1u32..=50) {
          let layout = CubicVoxelLayout::new(ChunkId(x1, y1), 1.0, voxel_length, voxel_length);
          let voxel = VoxelId(x2, 0, z2);
          let space_coords = layout.voxel_to_space(&voxel);
          let result = layout.space_to_voxel(&space_coords);
          assert_eq!(result, voxel, "Coords: {:?}", space_coords);
      }

      #[test]
      fn chunk_space_coordinates_should_be_reversible(x1 in -10000i32..=10000, y1 in -10000i32..=10000, x2 in -10000i32..=10000, z2 in -10000i32..=10000, voxel_length in 1u32..=50) {
          let layout = CubicVoxelLayout::new(ChunkId(x1, y1), 1.0, voxel_length, voxel_length);
          let voxel = VoxelId(x2, 0, z2);
          let chunk = layout.voxel_to_chunk(&voxel);
          let space_coords = layout.chunk_to_space(&chunk);
          let result = layout.space_to_chunk(&space_coords);
          assert_eq!(result, chunk, "Chunk coords: {:?}", space_coords);
      }

      #[test]
      fn voxel_should_resolve_to_same_chunk_in_space(x1 in -10000i32..=10000, y1 in -10000i32..=10000, x2 in -10000i32..=10000, z2 in -10000i32..=10000, voxel_length in 1u32..=50) {
          let layout = CubicVoxelLayout::new(ChunkId(x1, y1), 1.0, voxel_length, voxel_length);
          let voxel = VoxelId(x2, 0, z2);
          let space_coords = layout.voxel_to_space(&voxel);
          let space_chunk = layout.space_to_chunk(&space_coords);
          let voxel_chunk = layout.voxel_to_chunk(&voxel);
          assert_eq!(space_chunk, voxel_chunk);
      }

      #[test]
      fn voxel_to_chunk_xz_distance_should_be_voxel_length_or_less(x1 in -10000i32..=10000, y1 in -10000i32..=10000, x2 in -10000i32..=10000, z2 in -10000i32..=10000, voxel_length in 1u32..=50) {
          let layout = CubicVoxelLayout::new(ChunkId(x1, y1), 1.0, voxel_length, voxel_length);
          let voxel = VoxelId(x2, 0, z2);
          let chunk = layout.voxel_to_chunk(&voxel);
          let chunk_center = layout.get_center_voxel(&chunk);
          let diff = voxel - chunk_center;
          let distance = if diff.x() > diff.z() { diff.x() } else { diff.z() };
          assert!(distance <= layout.chunk_voxel_length as i32);
      }

      #[test]
      fn voxel_to_chunk_vertical_distance_should_be_voxel_length_or_less(x1 in -10000i32..=10000, y1 in -10000i32..=10000, x2 in -10000i32..=10000, z2 in -10000i32..=10000, voxel_length in 1u32..=50) {
          let layout = CubicVoxelLayout::new(ChunkId(x1, y1), 1.0, voxel_length, voxel_length);
          let voxel = VoxelId(x2, 0, z2);
          let chunk = layout.voxel_to_chunk(&voxel);
          let chunk_center = layout.get_center_voxel(&chunk);
          let diff = voxel - chunk_center;
          let distance = diff.y().abs();
          assert!(distance <= layout.chunk_voxel_length as i32);
      }

      #[test]
      fn voxel_to_chunk_should_return_same_value_for_same_chunk(x1 in -10000i32..=10000, y1 in -10000i32..=10000, ring_num in 0i32..10, index in 0i32..1000, voxel_length in 1u32..=50) {
          let layout = CubicVoxelLayout::new(ChunkId(x1, y1), 1.0, voxel_length, voxel_length);

          // find a random chunk via neighbors
          let mut chunk = ChunkId::default();
          for _ring in 0..ring_num {
              let mut n: Vec<_> = layout.get_chunk_neighbors(&chunk, 1);
              chunk = n.remove((index % n.len() as i32) as usize);
          }
          for voxel in layout.get_chunk_voxels(&chunk) {
              let result = layout.voxel_to_chunk(&voxel);
              assert_eq!(result, chunk, "Voxel: {:?}, expected chunk: {:?}, actual: {:?}", voxel, chunk, result);
          }
      }

      #[test]
      fn chunk_should_have_correct_number_of_voxels(x1 in -10000i32..=10000, y1 in -10000i32..=10000, x2 in -10000i32..=10000, z2 in -10000i32..=10000, voxel_length in 1u32..=50, height in 0u32..=50) {
          let layout = CubicVoxelLayout::new(ChunkId(x1, y1), 1.0, voxel_length, height);

          let voxel = VoxelId(x2, 0, z2);
          let chunk = layout.voxel_to_chunk(&voxel);
          let voxel_count = layout.get_chunk_voxels(&chunk).len() as i32;
          let expected = (layout.chunk_voxel_full_length() * layout.chunk_voxel_full_length()) * height; // 6 triangle cross-sections (excl center), each section has a number of voxels equal to the nth triangle number * height
          assert_eq!(expected as i32, voxel_count);
      }
  }
}
