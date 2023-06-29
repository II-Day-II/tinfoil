use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Color, CommandEncoder, Device,
    LoadOp, Operations, PipelineLayoutDescriptor, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, SamplerBindingType,
    ShaderModuleDescriptor, ShaderStages, SurfaceConfiguration,
    TextureSampleType, TextureView,
    TextureViewDimension,
};

use super::texture::Texture;

const RENDER_TEXTURE_SHADER: &str = r"
@group(0) @binding(0)
var screen: texture_2d<f32>;
@group(0) @binding(1)
var screen_sampler: sampler;

struct VertexToFragment {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vert_index: u32) -> VertexToFragment {
    // vertex positions in screen space (xy) and corresponding texture coordinates (zw)
    // texture coordinates are flipped so they correspond to OpenGL
    var QUAD_POSITIONS = array<vec4<f32>, 6>(
        vec4( 1.0,  1.0, 1.0, 1.0), // top right    (1.0, 0.0)
        vec4(-1.0, -1.0, 0.0, 0.0), // bottom left  (0.0, 1.0)
        vec4( 1.0, -1.0, 1.0, 0.0), // bottom right (1.0, 1.0)

        vec4(-1.0, -1.0, 0.0, 0.0), // bottom left  (0.0, 1.0)
        vec4( 1.0,  1.0, 1.0, 1.0), // top right    (1.0, 0.0)
        vec4(-1.0,  1.0, 0.0, 1.0), // top left     (0.0, 0.0)
    ); 
    let vert = QUAD_POSITIONS[vert_index];
    var out: VertexToFragment;
    out.position = vec4<f32>(vert.xy, 0.0, 1.0);
    out.tex_coord = vert.zw;
    return out;
}

@fragment
fn fs_main(in: VertexToFragment) -> @location(0) vec4<f32> {
    return textureSample(screen, screen_sampler, in.tex_coord);
}
";

pub(crate) struct RenderTexture {
    pipeline: RenderPipeline,
    bind_group: BindGroup,
    texture: Texture,
}

impl RenderTexture {
    pub fn from_texture(device: &Device, config: &SurfaceConfiguration, texture: Texture) -> Self {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("RenderTexture shader module"),
            source: wgpu::ShaderSource::Wgsl(RENDER_TEXTURE_SHADER.into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("RenderTexture bind group layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("RenderTexture pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("RenderTexture bind group"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&texture.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("RenderTexture pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        Self {
            pipeline,
            bind_group,
            texture,
        }
    }

    pub fn new(device: &Device, config: &SurfaceConfiguration) -> Self {
        let texture = Texture::render_texture(device, config);
        Self::from_texture(device, config, texture)
    }

    pub(crate) fn present(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("RenderTexture presentation render pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}
