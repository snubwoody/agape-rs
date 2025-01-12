use std::collections::HashMap;
use crate::{
    app::AppState,
    geometry::Vertex,
    resources::ResourceManager,
    view::View,
    Color
};


/// Draws a circle to the screen
/// 
/// # Example
/// ```
/// use helium::view::CircleView;
/// use helium::Color;
/// 
/// CircleView::new("")
/// 	.color(Color::default());
/// 
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CircleView {
    id: String,
    color: Color,
    resources: HashMap<String, usize>,
	vertices:Vec<Vertex>
}

impl CircleView {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            color: Color::default(),
            resources: HashMap::new(),
			vertices:vec![]
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl View for CircleView {
    fn id(&self) -> &str {
        &self.id
    }

    fn init(
        &mut self,
        layout: &dyn crystal::Layout,
        resources: &mut ResourceManager,
        state: &AppState,
    ) -> Result<(), crate::Error> {
		let diameter = layout.size().width;
		let position = layout.position();

		let vertices = Vertex::quad(layout.size(), position, self.color);

		let position_buffer = resources.add_uniform_init(
            "Circle Position Buffer",
            bytemuck::cast_slice(&[position.x,position.y]),
            &state.device,
        );

        let diameter_buffer = resources.add_uniform_init(
            "Circle Diamter Buffer",
            bytemuck::cast_slice(&[diameter]),
            &state.device,
        );

		let vertex_buffer = resources.add_vertex_buffer_init(
			"Vertex Buffer", 
			bytemuck::cast_slice(&vertices), 
			&state.device
		);

        let bind_group = resources
            .add_bind_group(
                "Circle Dimensions Bind Group",
                &state.context.circle_pipeline.bounds_layout,
                &state.device,
                &[diameter_buffer, position_buffer],
                &[],
                &[],
            )?;

		self.vertices = vertices;
		self.resources.insert("Bind group".to_string(), bind_group);
		self.resources.insert("Position".to_string(), position_buffer);
		self.resources.insert("Diameter".to_string(), diameter_buffer);
		self.resources.insert("Vertex buffer".to_string(), vertex_buffer);

        Ok(())
    }

    fn draw(
        &mut self,
        pass: &mut wgpu::RenderPass,
        resources: &ResourceManager,
        context: &crate::geometry::RenderContext,
        state: &AppState,
    ) {
		let vertex_buffer = resources.buffer(
			*self.resources.get("Vertex buffer").unwrap()
		).unwrap();

		let bind_group = resources.bind_group(
			*self.resources.get("Bind group").unwrap()
		).unwrap();

        pass.set_pipeline(&context.circle_pipeline.pipeline);
        pass.set_bind_group(0, &context.circle_pipeline.window_uniform.bind_group, &[]);
        pass.set_bind_group(1, bind_group, &[]);
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        pass.draw(0..self.vertices.len() as u32, 0..1);
    }
}
