use crate::{
    app::AppState,
    geometry::{vertex::Vertex, RenderContext},
    impl_surface,
    resources::ResourceManager,
    surface::Surface,
    Bounds, Color, Position, Size,
};
use helium_core::color::WHITE;
use wgpu::util::DeviceExt;

/// This is a primitive that draws to the screen. This holds
/// essential information about the [`Widget`], ie.
/// the color, coordinates and size.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CircleSurface {
    id: String,
    position: Position,
    size: Size,
    position_buffer: usize,
    diameter_buffer: usize,
    bind_group: usize,
    color: Color,
}

impl CircleSurface {
    pub fn new(id: &str, radius: u32, resources: &mut ResourceManager, state: &AppState) -> Self {
        let position_buffer = resources.add_uniform(
            "Circle Position Buffer",
            size_of::<[f64; 2]>().try_into().unwrap(),
            &state.device,
        );

        let diameter_buffer = resources.add_uniform(
            "Circle Diamter Buffer",
            size_of::<f64>().try_into().unwrap(),
            &state.device,
        );

        let bind_group = resources
            .add_bind_group(
                "Circle Dimensions Bind Group",
                &state.context.circle_pipeline.bounds_layout,
                &state.device,
                &[diameter_buffer,position_buffer],
                &[],
                &[],
            )
            .unwrap();

        let size = Size::new(radius as f32, radius as f32);
        let position = Position::default();

        Self {
            id: id.to_string(),
            position,
            size,
            position_buffer,
            diameter_buffer,
            bind_group,
            color: WHITE,
        }
    }

    pub fn color(&mut self, color: Color) {
        self.color = color
    }

    pub fn to_vertices(&self) -> Vec<Vertex> {
        let color = self.color.normalize();
		dbg!(color);
        let x = self.position.x;
        let y = self.position.y;

        let vertex1 = Vertex::new(x, y, color); //Top left
        let vertex2 = Vertex::new(x + self.size.width, y, color); // Top right
        let vertex3 = Vertex::new(x, y + self.size.height, color); //Bottom left
        let vertex4 = Vertex::new(x + self.size.width, y, color); //Top right
        let vertex5 = Vertex::new(x, y + self.size.height, color); // Bottom left
        let vertex6 = Vertex::new(x + self.size.width, y + self.size.height, color); //Bottom right

        return vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];
    }
}

impl Surface for CircleSurface {
    fn build(&mut self, state: &AppState, resources: &ResourceManager) {
		resources.write_buffer(
			self.position_buffer, 
			0, 
			bytemuck::cast_slice(&[self.position.x,self.position.y]), 
			&state.queue
		).unwrap();
		resources.write_buffer(
			self.diameter_buffer, 
			0, 
			bytemuck::cast_slice(&[self.diameter_buffer]), 
			&state.queue
		).unwrap();
	}

    fn draw(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        resources: &ResourceManager,
        context: &RenderContext,
        state: &AppState,
    ) {
        let vertices = self.to_vertices();

        let vertex_buffer = state
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        // Set the render pipeline and vertex buffer
        render_pass.set_pipeline(&context.circle_pipeline.pipeline);
        render_pass.set_bind_group(0, &context.circle_pipeline.window_uniform.bind_group(), &[]);
        render_pass.set_bind_group(1, resources.bind_group(self.bind_group).unwrap(), &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..vertices.len() as u32, 0..1);
    }

    impl_surface!();
}
