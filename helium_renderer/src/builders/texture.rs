use helium_core::Size;

pub struct TextureBuilder<'b> {
    label: Option<&'b str>,
    size: Size,
    mip_level_count: u32,
    sample_count: u32,
    dimension: wgpu::TextureDimension,
    format: wgpu::TextureFormat,
    usage: wgpu::TextureUsages,
}

impl<'b> TextureBuilder<'b> {
    pub fn new() -> Self {
        Self {
            label: None,
            size: Size::default(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::empty(),
        }
    }

    /// Set the label of the texture
    pub fn label(mut self, label: &'b str) -> Self {
        self.label = Some(label);
        self
    }

    /// Set the size of the texture
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Set the dimensions of the texture
    pub fn dimension(mut self, dimension: wgpu::TextureDimension) -> Self {
        self.dimension = dimension;
        self
    }

    /// Set the texture format
    pub fn format(mut self, format: wgpu::TextureFormat) -> Self {
        self.format = format;
        self
    }

    /// Set the texture usage
    pub fn usage(mut self, usage: wgpu::TextureUsages) -> Self {
        self.usage |= usage;
        self
    }

    /// Build the `wgpu::Texture`
    pub fn build(self, device: &wgpu::Device) -> wgpu::Texture {
        device.create_texture(&wgpu::TextureDescriptor {
            label: self.label,
            size: wgpu::Extent3d {
                width: self.size.width as u32,
                height: self.size.height as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: self.mip_level_count,
            sample_count: self.sample_count,
            dimension: self.dimension,
            format: self.format,
            usage: self.usage,
            view_formats: &[],
        })
    }
}
