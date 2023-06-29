use wgpu::{
    AddressMode, Device, Extent3d, FilterMode, Sampler, SamplerDescriptor, SurfaceConfiguration,
    TextureDescriptor, TextureDimension, TextureUsages, TextureView, TextureViewDescriptor,
};

pub struct Texture {
    texture: wgpu::Texture,
    pub view: TextureView,
    pub sampler: Sampler,
}

impl Texture {
    pub fn render_texture(device: &Device, config: &SurfaceConfiguration) -> Self {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("RenderTexture texture"),
            size: Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: config.format,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING, // may need RENDER_ATTACHMENT?
            view_formats: &[],
        });
        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("RenderTexture sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });
        Self {
            texture,
            view,
            sampler,
        }
    }
}
