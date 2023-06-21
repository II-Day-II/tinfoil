use wgpu::{CommandEncoder, ComputePassDescriptor, ComputePipeline, BindGroup};


pub struct UserShaders {
    pipeline: ComputePipeline,
    bind_groups: Vec<BindGroup>,
    workgroups: (u32, u32, u32),
}

impl UserShaders {
    pub fn execute(&self, encoder: &mut CommandEncoder) {
        let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("UserShader compute pass"),
        });
        compute_pass.set_pipeline(&self.pipeline);
        for (i, group) in self.bind_groups.iter().enumerate() {
            compute_pass.set_bind_group(i as u32, group, &[]);
        }
        let (x, y, z) = self.workgroups;
        compute_pass.dispatch_workgroups(x, y, z)
    }
}