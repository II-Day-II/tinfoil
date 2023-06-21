use wgpu::{
    Backends, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits,
    PowerPreference, PresentMode, Queue,
    RequestAdapterOptions, Surface, SurfaceConfiguration,
    SurfaceError, TextureUsages,
};
use winit::{dpi::PhysicalSize, window::Window};

use super::{render_texture::RenderTexture, texture::Texture, user_shaders::UserShaders};

pub const VSYNC: bool = true;

pub(crate) struct Graphics {
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    window: Window,
    size: PhysicalSize<u32>,
    render_texture: RenderTexture,
    user_shaders: Option<UserShaders>,
}

impl Graphics {
    pub(crate) async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });
        // safety: this needs to have lifetime >= window that created it. both are owned by Self, so this is fine.
        let surface = unsafe {
            instance
                .create_surface(&window)
                .expect("couldn't create surface")
        };

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("couldn't get GPU adapter");

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: if VSYNC {
                PresentMode::AutoVsync
            } else {
                PresentMode::AutoNoVsync
            },
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("Initial device request descriptor"),
                    features: Features::empty(),
                    limits: Limits::default(),
                },
                None,
            )
            .await
            .expect("Couldn't get device and queue");

        let base_texture = Texture::new(&device, &config);
        let render_texture = RenderTexture::from_texture(&device, &config, base_texture);

        Self {
            surface,
            device,
            queue,
            config,
            window,
            size,
            render_texture,
            user_shaders: None,
        }
    }
    pub fn window(&self) -> &Window {
        &self.window
    }
    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        
    }
    pub fn render(&self) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Main Encoder"),
            });

        // raytrace the scene to the render texture
        if let Some(shaders) = &self.user_shaders {
            shaders.execute(&mut encoder);
        }

        // show the render texture on the screen
        self.render_texture.present(&mut encoder, &view);

        self.queue.submit([encoder.finish()]); // tell the GPU to do all the things
        output.present(); // present the final image to the screen

        Ok(())
    }
}
