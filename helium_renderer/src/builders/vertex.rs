/// Builder pattern for `wgpu::VertexBufferLayout`.
///
/// # Example
/// ```
/// use std::mem;
/// use helium_renderer::builders::VertexBufferLayoutBuilder;
///
/// let vertex_layout = VertexBufferLayoutBuilder::new()
/// 	.array_stride(128)
/// 	.step_mode(wgpu::VertexStepMode::Vertex)
/// 	.attribute(mem::size_of::<[f32;2]>() as u64,wgpu::VertexFormat::Float32x2)
/// 	.attribute(mem::size_of::<[f32;4]>() as u64,wgpu::VertexFormat::Float32x4)
/// 	.attribute(mem::size_of::<[f32;4]>() as u64,wgpu::VertexFormat::Float32x2)
/// 	.build();
///
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VertexBufferLayoutBuilder {
    array_stride: u64,
    step_mode: wgpu::VertexStepMode,
    attributes: Vec<wgpu::VertexAttribute>,
}

impl VertexBufferLayoutBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn array_stride(mut self, array_stride: u64) -> Self {
        self.array_stride = array_stride;
        self
    }

    pub fn step_mode(mut self, step_mode: wgpu::VertexStepMode) -> Self {
        self.step_mode = step_mode;
        self
    }

    /// Add `wgpu::VertexAttribute` to the `VertexBufferLayout`. The shader
    /// location of the attributes will be in the order they are defined.
    /// ```
    /// use std::mem;
    /// use helium_renderer::builders::VertexBufferLayoutBuilder;
    ///
    /// VertexBufferLayoutBuilder::new()
    /// 	.attribute(mem::size_of::<[f32;2]>() as u64,wgpu::VertexFormat::Float32x2) // 0
    /// 	.attribute(mem::size_of::<[f32;4]>() as u64,wgpu::VertexFormat::Float32x4) // 1
    /// 	.attribute(mem::size_of::<[f32;2]>() as u64,wgpu::VertexFormat::Float32x2) // 2
    /// 	.build();
    /// ```
    pub fn attribute(mut self, offset: u64, format: wgpu::VertexFormat) -> Self {
        let attribute = wgpu::VertexAttribute {
            offset,
            shader_location: self.attributes.len() as u32,
            format,
        };

        self.attributes.push(attribute);
        self
    }

    pub fn build(self) -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: self.array_stride,
            step_mode: self.step_mode,
            attributes: Box::leak(self.attributes.into_boxed_slice()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shader_location_matches_order() {
        let layout = VertexBufferLayoutBuilder::new()
            .attribute(12, wgpu::VertexFormat::Float16x2)
            .attribute(54, wgpu::VertexFormat::Float16x2)
            .attribute(0, wgpu::VertexFormat::Float16x2)
            .attribute(100, wgpu::VertexFormat::Float16x2)
            .attribute(0, wgpu::VertexFormat::Float16x2);

        assert!(layout.attributes[0].shader_location == 0);
        assert!(layout.attributes[1].shader_location == 1);
        assert!(layout.attributes[2].shader_location == 2);
        assert!(layout.attributes[3].shader_location == 3);
        assert!(layout.attributes[4].shader_location == 4);
    }
}
