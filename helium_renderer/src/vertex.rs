use bytemuck::{Pod, Zeroable};
use helium_core::color::Color;
use helium_core::{Position, Size};

/// Represents a single vertex with a 2D position, color and uv coordinates.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Pod, Default, Zeroable)]
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

    /// Creates a `Vec` of 6 `Vertices` in a quad layout.
    ///
    /// # Example
    /// ```
    /// use helium_core::{Size,Position,Color};
    /// use helium_renderer::Vertex;
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

    /// Creates a `Vec` of 6 `Vertices` in a quad layout allowing you to specify 
	/// uv coordinates.
	/// The uv's are defined clockwise
	/// - \[0\] - Top left
	/// - \[1\] - Top right
	/// - \[2\] - Bottom right
	/// - \[3\] - Bottom left
    ///
    /// # Example
    /// ```
    /// use helium_core::{Size,Position,Color};
    /// use helium_renderer::Vertex;
    ///
    /// let size = Size::new(50.0,75.0);
    /// let position = Position::default();
    /// let color = Color::default();
    /// let uv = [
	/// 	[0.0,0.0], // Top left
	/// 	[1.0,0.0], // Top right
	/// 	[1.0,1.0], // Bottom right
	/// 	[0.0,1.0], // Bottom left
	/// ];
    /// let vertices = Vertex::quad_with_uv(
	/// 	size,
	/// 	position,
	/// 	color,
	/// 	uv
	/// );
    ///
    /// assert_eq!(vertices[0].position[0],position.x);
    /// assert_eq!(vertices[5].position[0],position.x + size.width);
    /// assert_eq!(vertices[0].uv,uv[0]);
    /// assert_eq!(vertices[5].uv,uv[2]);
    /// ```
    pub fn quad_with_uv(
		size: Size, 
		position: Position, 
		color: Color,
		uv: [[f32; 2];4]
	) -> Vec<Self> {
        let color = color.normalize();
        let width = size.width;
        let height = size.height;
        let x = position.x;
        let y = position.y;

        let vertex1 = Vertex::new_with_uv(x, y, color, uv[0]); //Top left
        let vertex2 = Vertex::new_with_uv(x + width, y, color, uv[1]); // Top right
        let vertex3 = Vertex::new_with_uv(x, y + height, color, uv[3]); //Bottom left
        let vertex4 = Vertex::new_with_uv(x + width, y, color, uv[1]); //Top right
        let vertex5 = Vertex::new_with_uv(x, y + height, color, uv[3]); // Bottom left
        let vertex6 = Vertex::new_with_uv(x + width, y + height, color, uv[2]); //Bottom right

        return vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];
    }
}
