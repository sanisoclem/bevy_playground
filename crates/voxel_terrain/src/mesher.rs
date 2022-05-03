use super::generator::VoxelType;
use bevy::{
  prelude::*,
  render::{
    mesh::{Indices, VertexAttributeValues},
    render_resource::PrimitiveTopology,
  },
  tasks::{AsyncComputeTaskPool, Task},
};
use block_mesh::{
  greedy_quads,
  ndshape::{RuntimeShape, Shape},
  GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG,
};

// TODO: lod
// TODO: use asset loader and return Handle<Mesh> instead of blocking
// pub fn generate_mesh(
//   thread_pool: &Res<AsyncComputeTaskPool>,
//   voxels: &[VoxelType],
//   shape: RuntimeShape<u32, 3>,
//   _lod: u8,
// ) -> Task<Mesh> {
//   // how do we use the voxel data?
//   // we cannot move the voxel data out of the ecs system
//   // for now we could clone it but maybe the voxel data needs to sit somewhere else
//   // but! if it's not in the ecs, how do we edit the voxel data from a system?
//   // and if we can edit, we need to make sure that we don't edit while we are using it to generate
//   // the mesh hmmm... maybe we need some sort of double buffer?
//   // edits are made in the front buffer while we use the back buffer to generate the mesh
//   // we swap buffers if there are changes in the front buffer and mesh generation is complete
//   let v = voxels.iter().cloned().collect::<Vec<_>>();

//   thread_pool.spawn(async move {
//     let scale = 1.0;
//     let mut mesh_buffer = GreedyQuadsBuffer::new(shape.usize());

//     let [x, y, z] = shape.as_array();
//     greedy_quads(
//       &v,
//       &shape,
//       [0; 3],
//       [x - 1, y - 1, z - 1],
//       &RIGHT_HANDED_Y_UP_CONFIG.faces,
//       &mut mesh_buffer,
//     );

//     let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

//     let num_indices = mesh_buffer.quads.num_quads() * 6;
//     let num_vertices = mesh_buffer.quads.num_quads() * 4;
//     let mut indices = Vec::with_capacity(num_indices);
//     let mut positions = Vec::with_capacity(num_vertices);
//     let mut normals = Vec::with_capacity(num_vertices);
//     let mut uvs = Vec::with_capacity(num_vertices);

//     for (_, (group, face)) in mesh_buffer
//       .quads
//       .groups
//       .as_ref()
//       .into_iter()
//       .zip(RIGHT_HANDED_Y_UP_CONFIG.faces.into_iter())
//       .enumerate()
//     {
//       for quad in group.into_iter() {
//         let i = face.quad_mesh_indices(positions.len() as u32);
//         let p = face.quad_mesh_positions(&quad, scale);
//         let n = face.quad_mesh_normals(); // calculate_normals(&p, &i);

//         indices.extend_from_slice(&i);
//         positions.extend_from_slice(&p);
//         normals.extend_from_slice(&n);
//         uvs.extend_from_slice(&face.tex_coords(RIGHT_HANDED_Y_UP_CONFIG.u_flip_face, false, &quad));
//         // uvs.extend_from_slice(&[
//         //   [0., 0.],
//         //   [0., 1.],
//         //   [1., 1.],
//         //   [1., 0.]
//         // ]);
//       }
//     }

//     mesh.insert_attribute(
//       Mesh::ATTRIBUTE_POSITION,
//       VertexAttributeValues::Float32x3(positions),
//     );

//     mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
//     mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
//     mesh.set_indices(Some(Indices::U32(indices)));

//     mesh
//   })
// }

pub fn generate_mesh(
  thread_pool: &Res<AsyncComputeTaskPool>,
  voxels: &[f32],
  shape: RuntimeShape<u32, 3>,
  _lod: u8,
) -> Task<Mesh> {
  let v = voxels.iter().cloned().collect::<Vec<_>>();

  thread_pool.spawn(async move {
    let scale = 1.0;

    let [x, y, z] = shape.as_array();
    let mut buffer = fast_surface_nets::SurfaceNetsBuffer::default();
    fast_surface_nets::surface_nets(v.as_slice(), &shape,
      [0; 3],
      [x - 1, y - 1, z - 1], &mut buffer);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let num_vertices = buffer.positions.len();

    mesh.insert_attribute(
      Mesh::ATTRIBUTE_POSITION,
      VertexAttributeValues::Float32x3(buffer.positions),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, buffer.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0; 2]; num_vertices]);
    mesh.set_indices(Some(Indices::U32(buffer.indices)));

    mesh
  })
}
