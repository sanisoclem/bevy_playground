use bevy::{
  prelude::*,
  reflect::TypeUuid,
  render::{
    pipeline::{PipelineDescriptor, RenderPipeline},
    render_graph::{base, AssetRenderResourcesNode, RenderGraph},
    renderer::RenderResources,
    shader::{ShaderStage, ShaderStages},
  },
};

#[derive(Debug, Clone, Default, TypeUuid)]
#[uuid = "4ee9c363-1124-4113-890e-199d81b00281"]
pub struct SkyMaterial {
  pub texture: Handle<Texture>,
  pub base_color: Vec4,
}

#[derive(Clone)]
pub struct GpuSkyMaterial {
  _buffer: Buffer,
  bind_group: BindGroup,
}

impl RenderAsset for SkyMaterial {
  type ExtractedAsset = CustomMaterial;
  type PreparedAsset = GpuSkyMaterial;
  type Param = (
    SRes<RenderDevice>,
    SRes<MaterialPipeline<Self>>,
    SRes<RenderAssets<Image>>,
  );
  fn extract_asset(&self) -> Self::ExtractedAsset {
    self.clone()
  }

  fn prepare_asset(
    material: Self::ExtractedAsset,
    (render_device, pbr_pipeline, gpu_images): &mut SystemParamItem<Self::Param>,
  ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
    let (tex_view, tex_sampler) = if let Some(result) = pbr_pipeline
      .mesh_pipeline
      .get_image_texture(gpu_images, &material.texture)
    {
      result
    } else {
      return Err(PrepareAssetError::RetryNextUpdate(material));
    };

    let base_color = Vec4::from_slice(&extracted_asset.color.as_linear_rgba_f32());
    let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
      contents: base_color.as_std140().as_bytes(),
      label: None,
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });
    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
      entries: &[
        BindGroupEntry {
          binding: 0,
          resource: buffer.as_entire_binding(),
        },
        BindGroupEntry {
          binding: 1,
          resource: BindingResource::TextureView(tex_view),
        },
        BindGroupEntry {
          binding: 2,
          resource: BindingResource::Sampler(tex_sampler),
        },
      ],
      label: None,
      layout: &pbr_pipeline.material_layout,
    });

    Ok(GpuSkyMaterial {
      _buffer: buffer,
      bind_group,
    })
  }
}

impl Material for CustomMaterial {
  fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
    Some(asset_server.load("shaders/sky_material.wgsl"))
  }

  fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
    &render_asset.bind_group
  }

  fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
    render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
      entries: &[
        BindGroupLayoutEntry {
          binding: 0,
          visibility: ShaderStages::FRAGMENT,
          ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: BufferSize::new(Vec4::std140_size_static() as u64),
          },
          count: None,
        },
        BindGroupLayoutEntry {
          binding: 1,
          visibility: ShaderStages::FRAGMENT,
          ty: BindingType::Texture {
            multisampled: false,
            sample_type: TextureSampleType::Float { filterable: true },
            view_dimension: TextureViewDimension::D2,
          },
          count: None,
        },
        BindGroupLayoutEntry {
          binding: 2,
          visibility: ShaderStages::FRAGMENT,
          ty: BindingType::Sampler(SamplerBindingType::Filtering),
          count: None,
        },
      ],
      label: None,
    })
  }
}
