/// Builder pattern for `wgpu::BindGroupLayout`
///
/// The binding of the entries will be in the order they are
/// defined.
/// ```no_run
/// use helium_renderer::builders::BindGroupLayoutBuilder;
///
/// BindGroupLayoutBuilder::new()
/// 	.uniform(wgpu::ShaderStages::FRAGMENT) // 0
/// 	.uniform(wgpu::ShaderStages::VERTEX) // 1
/// 	.uniform(wgpu::ShaderStages::VERTEX_FRAGMENT); // 2
/// ```
#[derive(Debug, Clone, Default)]
pub struct BindGroupLayoutBuilder<'b> {
    label: Option<&'b str>,
    entries: Vec<wgpu::BindGroupLayoutEntry>,
}

impl<'b> BindGroupLayoutBuilder<'b> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: &'b str) -> Self {
        self.label = Some(label);
        self
    }

    /// Add a uniform buffer to the layout entries
    pub fn uniform(mut self, visibility: wgpu::ShaderStages) -> Self {
        let entry = wgpu::BindGroupLayoutEntry {
            binding: self.entries.len() as u32,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };

        self.entries.push(entry);
        self
    }

    /// Add a sampler to the layout entries
    pub fn sampler(mut self, visibility: wgpu::ShaderStages, ty: wgpu::SamplerBindingType) -> Self {
        let entry = wgpu::BindGroupLayoutEntry {
            binding: self.entries.len() as u32,
            visibility,
            ty: wgpu::BindingType::Sampler(ty),
            count: None,
        };

        self.entries.push(entry);
        self
    }

    /// Add a sampler to the layout entries
    pub fn texture(
        mut self,
        visibility: wgpu::ShaderStages,
        ty: wgpu::TextureSampleType,
        dimension: wgpu::TextureViewDimension,
        multisampled: bool,
    ) -> Self {
        let entry = wgpu::BindGroupLayoutEntry {
            binding: self.entries.len() as u32,
            visibility,
            ty: wgpu::BindingType::Texture {
                sample_type: ty,
                view_dimension: dimension,
                multisampled,
            },
            count: None,
        };

        self.entries.push(entry);
        self
    }

    pub fn build(self, device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: self.label,
            entries: &self.entries,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct BindGroupBuilder<'b> {
    label: Option<&'b str>,
    entries: Vec<wgpu::BindGroupEntry<'b>>,
}

impl<'b> BindGroupBuilder<'b> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: &'b str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn buffer(mut self, buffer: &'b wgpu::Buffer) -> Self {
        let entry = wgpu::BindGroupEntry {
            binding: self.entries.len() as u32,
            resource: buffer.as_entire_binding(),
        };

        self.entries.push(entry);
        self
    }

    pub fn sampler(mut self, sampler: &'b wgpu::Sampler) -> Self {
        let entry = wgpu::BindGroupEntry {
            binding: self.entries.len() as u32,
            resource: wgpu::BindingResource::Sampler(&sampler),
        };

        self.entries.push(entry);
        self
    }

    pub fn texture_view(mut self, texture_view: &'b wgpu::TextureView) -> Self {
        let entry = wgpu::BindGroupEntry {
            binding: self.entries.len() as u32,
            resource: wgpu::BindingResource::TextureView(texture_view),
        };

        self.entries.push(entry);
        self
    }

    pub fn build(
        self,
        layout: &wgpu::BindGroupLayout,
        device: &'b wgpu::Device,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: self.label,
            entries: &self.entries,
            layout,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builders::BufferBuilder;
    use crate::setup;

    #[tokio::test]
    async fn bind_group_layout_builder() {
        let (device, _) = setup().await;

        let _ = BindGroupLayoutBuilder::new()
            .label("Bind group layout")
            .uniform(wgpu::ShaderStages::FRAGMENT)
            .sampler(
                wgpu::ShaderStages::all(),
                wgpu::SamplerBindingType::Filtering,
            )
            .texture(
                wgpu::ShaderStages::VERTEX,
                wgpu::TextureSampleType::Depth,
                wgpu::TextureViewDimension::D2,
                false,
            )
            .build(&device);
    }

    #[tokio::test]
    async fn bind_group_builder() {
        let (device, _) = setup().await;

        let size = BufferBuilder::new()
            .uniform()
            .init(&[200.0, 200.0])
            .copy_dst()
            .uniform()
            .build(&device);

        let color = BufferBuilder::new()
            .uniform()
            .init(&[200.0, 255.0, 30.0, 100.0])
            .copy_dst()
            .uniform()
            .build(&device);

        let layout = BindGroupLayoutBuilder::new()
            .label("Bind group layout")
            .uniform(wgpu::ShaderStages::FRAGMENT)
            .uniform(wgpu::ShaderStages::FRAGMENT)
            .build(&device);

        let _ = BindGroupBuilder::new()
            .label("Bind group")
            .buffer(&color)
            .buffer(&size)
            .build(&layout, &device);
    }
}
