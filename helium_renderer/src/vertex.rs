use helium_core::{Position, Size};
use helium_core::color::Color;
use bytemuck::{Pod,Zeroable};

/// Represents a single vertex
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq,PartialOrd, Pod, Default,Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub uv: [f32; 2],
}

/// TODO maybe remove color from the constructor
impl Vertex {
    /// Creates a new [`Vertex`]
    pub fn new(x: f32, y: f32, color: [f32; 4]) -> Self {
        Self {
            position: [x, y],
            color,
            uv: [1.0, 1.0],
        }
    }

    /// Creates a new [`Vertex`] with texture uv's.
    pub fn new_with_uv(x: f32, y: f32, color: [f32; 4], uv: [f32; 2]) -> Self {
        let r = color[0];
        let g = color[1];
        let b = color[2];
        let a = color[3];

        Self {
            position: [x, y],
            color: [r, g, b, a],
            uv,
        }
    }

    /// Creates a `Vec` of `Vertices` in a quad layout.
    ///
    /// # Example
    /// ```
	/// use helium_core::{Size,Position,Color};
    /// use helium_renderer::{vertex::Vertex,};
    ///
    /// let size = Size::new(50.0,75.0);
    /// let position = Position::default();
    /// let color = Color::default();
    ///
    /// let vertices = Vertex::quad(size,position,color);
    ///
    /// assert_eq!(vertices[0].position[0],position.x);
    /// assert_eq!(vertices[5].position[0],position.x + size.width);
    /// ```
    pub fn quad(size: Size, position: Position, color: Color) -> Vec<Self> {
        let color = color.normalize();
        let width = size.width;
        let height = size.height;
        let x = position.x;
        let y = position.y;

        let vertex1 = Vertex::new_with_uv(x, y, color, [0.0, 0.0]); //Top left
        let vertex2 = Vertex::new_with_uv(x + width, y, color, [1.0, 0.0]); // Top right
        let vertex3 = Vertex::new_with_uv(x, y + height, color, [0.0, 1.0]); //Bottom left
        let vertex4 = Vertex::new_with_uv(x + width, y, color, [1.0, 0.0]); //Top right
        let vertex5 = Vertex::new_with_uv(x, y + height, color, [0.0, 1.0]); // Bottom left
        let vertex6 = Vertex::new_with_uv(x + width, y + height, color, [1.0, 1.0]); //Bottom right

        return vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];
    }

    pub fn tri(size: Size, position: Position, color: Color) -> Vec<Self> {
		// TODO add orientation option
        let color = color.normalize();
        let width = size.width;
        let height = size.height;
        let x = position.x;
        let y = position.y;

        let vertex1 = Vertex::new_with_uv(x, y, color, [0.0, 0.0]); //Top left
        let vertex2 = Vertex::new_with_uv(x + width, y, color, [1.0, 0.0]); // Top right
        let vertex3 = Vertex::new_with_uv(x, y + height, color, [0.0, 1.0]); //Bottom left

        return vec![vertex1, vertex2, vertex3,];
    }
}

// TODO move to resources?
pub struct VertexBufferLayoutBuilder {
    attributes: Vec<wgpu::VertexAttribute>,
}

impl VertexBufferLayoutBuilder {
    pub fn new() -> Self {
        Self { attributes: vec![] }
    }

    /// Adds a vertex attribute to the `VertexBufferLayout`
    pub fn add_attribute(mut self, offset: usize, format: wgpu::VertexFormat) -> Self {
        let shader_location = self.attributes.len() as u32;
        let attribute = wgpu::VertexAttribute {
            offset: offset as wgpu::BufferAddress,
            shader_location,
            format,
        };
        self.attributes.push(attribute);
        self
    }

    pub fn build(self) -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: Box::leak(Box::new(self.attributes)),
        }
    }
}
