use bytemuck::NoUninit;
use wgpu::util::DeviceExt;

/// Builder pattern for creating `wgpu::Buffer`'s.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferBuilder<'a> {
    label: Option<&'a str>,
    size: Option<u64>,
    buffer_usages: wgpu::BufferUsages,
    contents: Option<&'a [u8]>,
    mapped_at_creation: bool,
}

impl<'a> BufferBuilder<'a> {
    pub fn new() -> Self {
        Self {
            label: None,
            size: None,
            buffer_usages: wgpu::BufferUsages::empty(),
            contents: None,
            mapped_at_creation: false,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Set the size of the buffer, the size must be
    /// set if no initialization data is provided, the builder
    /// will panic if there is no size and no data initilization
    /// data.
    pub fn size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    pub fn mapped_at_creation(mut self, val: bool) -> Self {
        self.mapped_at_creation = val;
        self
    }

    pub fn uniform(mut self) -> Self {
        self.buffer_usages |= wgpu::BufferUsages::UNIFORM;
        self
    }

    /// Set the buffer usage to `BufferUsages::COPY_DST`.
    pub fn copy_dst(mut self) -> Self {
        self.buffer_usages |= wgpu::BufferUsages::COPY_DST;
        self
    }

    /// Set the buffer usage to `BufferUsages::VERTEX`.
    pub fn vertex(mut self) -> Self {
        self.buffer_usages |= wgpu::BufferUsages::VERTEX;
        self
    }

    pub fn usage(mut self, usage: wgpu::BufferUsages) -> Self {
        self.buffer_usages |= usage;
        self
    }

    /// Provide data to initialise the `Buffer`.
    pub fn init<A>(mut self, contents: &'a [A]) -> Self
    where
        A: NoUninit,
    {
        let contents: &[u8] = bytemuck::cast_slice(&contents);
        self.contents = Some(contents);
        self
    }

    pub fn build(self, device: &wgpu::Device) -> wgpu::Buffer {
        match self.contents {
            Some(data) => {
                return device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: self.label,
                    usage: self.buffer_usages,
                    contents: &data,
                });
            }
            None => {
                return device.create_buffer(&wgpu::BufferDescriptor {
                    label: self.label,
                    usage: self.buffer_usages,
                    size: self.size.unwrap(),
                    mapped_at_creation: self.mapped_at_creation,
                });
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::setup;

    #[tokio::test]
    async fn buffer_builder() {
        let (device, _) = setup().await;

        let _ = BufferBuilder::new()
            .size(24)
            .uniform()
            .copy_dst()
            .build(&device);
    }

    #[tokio::test]
    #[should_panic]
    async fn build_fails_with_no_size() {
        let (device, _) = setup().await;

        let _ = BufferBuilder::new().copy_dst().build(&device);
    }
}
