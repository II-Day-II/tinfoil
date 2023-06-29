use wgpu::{CommandEncoder, ComputePassDescriptor, ComputePipeline, BindGroup, Device};

pub struct UserShadersBuilder {
    shaders: Vec<Shader>,
}
impl UserShadersBuilder {
    pub(super) fn build(self, device: &Device, ) -> UserShaders {
        todo!();
    }
    pub fn new() -> Self {
        Self {
            shaders: Vec::new(),
        }
    }
    pub fn with_shader(&mut self, shader: Shader) -> &mut Self {
        todo!();
        self
    }
}

pub struct Shader {
    pipeline: ComputePipeline,
    bind_groups: Vec<BindGroup>,
    workgroups: (u32, u32, u32),
}

pub(super) struct UserShaders {
    shaders: Vec<Shader>,
}

impl UserShaders {
    pub fn execute(&self, encoder: &mut CommandEncoder) {
        let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("UserShader compute pass"),
        });
        for shader in &self.shaders {
            compute_pass.set_pipeline(&shader.pipeline);
            for (i, group) in shader.bind_groups.iter().enumerate() {
                compute_pass.set_bind_group(i as u32, group, &[]);
            }
            let (x, y, z) = shader.workgroups;
            compute_pass.dispatch_workgroups(x, y, z)

        }
    }
}