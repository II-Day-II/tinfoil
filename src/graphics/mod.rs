use wgpu::{Surface, SurfaceConfiguration, Device, Queue, Instance, InstanceDescriptor, Backends, RequestAdapterOptions, PowerPreference, DeviceDescriptor, Features, Limits, TextureUsages, PresentMode};
use winit::{window::Window, dpi::PhysicalSize};
pub struct Graphics {
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    window: Window,
    size: PhysicalSize<u32>,
}
pub const VSYNC: bool = true;

impl Graphics {
    pub async fn new(window: Window) -> Self {

        let size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });
        // safety: this needs to have lifetime >= window that created it. both are owned by Self, so this is fine.
        let surface = unsafe { instance.create_surface(&window).expect("couldn't create surface") };

        let adapter = instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.expect("couldn't get GPU adapter");

        let surface_caps = surface.get_capabilities(&adapter);
        
        let surface_format = surface_caps.formats.iter().copied().filter(|f| f.is_srgb()).next().unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration { 
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: if VSYNC {PresentMode::AutoVsync} else {PresentMode::AutoNoVsync},
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        let (device, queue) = adapter.request_device(
            &DeviceDescriptor {
                label: Some("Initial device request descriptor"),
                features: Features::empty(),
                limits: Limits::default(),
            }, 
            None,
        ).await.expect("Couldn't get device and queue");


        Self {
            surface,
            device,
            queue,
            config,
            window,
            size
        }
    }
    pub fn window(&self) -> &Window {
        &self.window
    }
    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }
}