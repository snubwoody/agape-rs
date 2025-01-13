use crate::{app::AppState, geometry::Vertex, resources::ResourceManager, Color};
use std::collections::HashMap;

use super::View;

#[derive(Debug, Clone, PartialEq)]
pub struct RectView {
    id: String,
    color: Color,
    corner_radius: u32,
    resources: HashMap<String, usize>,
    vertices: Vec<Vertex>,
}

impl RectView {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            color: Color::default(),
            corner_radius: 0,
            resources: HashMap::new(),
            vertices: vec![],
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.corner_radius = corner_radius;
        self
    }
}

impl View for RectView {
    fn id(&self) -> &str {
        &self.id
    }

    fn init(
        &mut self,
        layout: &dyn crystal::Layout,
        resources: &mut ResourceManager,
        state: &AppState,
    ) -> Result<(), crate::Error> {
        let size = layout.size();
        let position = layout.position();

        let vertices = Vertex::quad(size, position, self.color);

        let vertex_buffer = resources.add_vertex_buffer_init(
            "Rect Vertex Buffer",
            bytemuck::cast_slice(&vertices),
            &state.device,
        );

        let size_buffer = resources.add_uniform_init(
            "Rect Size Buffer",
            bytemuck::cast_slice(&[size.width, size.height]),
            &state.device,
        );

        let position_buffer = resources.add_uniform_init(
            "Rect Position Buffer",
            bytemuck::cast_slice(&[position.x, position.y]),
            &state.device,
        );

        let radius_buffer = resources.add_uniform_init(
            "Rect Corner Radius Buffer",
            bytemuck::cast_slice(&[self.corner_radius]),
            &state.device,
        );

        let bind_group = resources.add_bind_group(
            "Rect Bind Group",
            &state.context.rect_pipeline.bounds_layout,
            &state.device,
            &[radius_buffer, size_buffer, position_buffer],
            &[],
            &[],
        )?;

        self.resources
            .insert("Vertex buffer".to_string(), vertex_buffer);
        self.resources.insert("Size".to_string(), size_buffer);
        self.resources
            .insert("Position".to_string(), position_buffer);
        self.resources
            .insert("Size".to_string(), size_buffer);
        self.resources
            .insert("Corner radius".to_string(), radius_buffer);
        self.resources.insert("Bind group".to_string(), bind_group);
        self.vertices = vertices;

        Ok(())
    }

    fn resize(
		&mut self, 
		layout: &dyn crystal::Layout, 
		resources: &ResourceManager, 
		state: &AppState
	) -> Result<(),crate::Error> {
		let position = layout.position();
		let size = layout.size();

		self.vertices = Vertex::quad(size, position, self.color);

		let vertex_buffer = self.resources.get("Vertex buffer").unwrap();
		let position_buffer = self.resources.get("Position").unwrap();
		let size_buffer = self.resources.get("Size").unwrap();
		
		resources.write_buffer(
			*vertex_buffer, 
			0, 
			bytemuck::cast_slice(&self.vertices), 
			&state.queue
		)?;
		resources.write_buffer(
			*position_buffer, 
			0, 
			bytemuck::cast_slice(&[position.x,position.y]), 
			&state.queue
		)?;
		resources.write_buffer(
			*size_buffer, 
			0, 
			bytemuck::cast_slice(&[size.width,size.height]), 
			&state.queue
		)?;

		Ok(())
    }

    fn draw(
        &mut self,
        pass: &mut wgpu::RenderPass,
        resources: &ResourceManager,
        context: &crate::geometry::RenderContext,
        state: &AppState,
    ) {
        let vertex_buffer = resources
            .buffer(*self.resources.get("Vertex buffer").unwrap())
            .unwrap();
        let bind_group = resources
            .bind_group(*self.resources.get("Bind group").unwrap())
            .unwrap();

        pass.set_pipeline(&context.rect_pipeline.pipeline);
        pass.set_bind_group(0, &context.rect_pipeline.window_bind_group, &[]);
        pass.set_bind_group(1, bind_group, &[]);
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        pass.draw(0..self.vertices.len() as u32, 0..1);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crystal::EmptyLayout;
    use winit::{platform::windows::EventLoopBuilderExtWindows, window::Window};

    fn window() -> Window {
        let event_loop = winit::event_loop::EventLoopBuilder::new()
            .with_any_thread(true)
            .build()
            .unwrap();

        winit::window::WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap()
    }

    #[tokio::test]
    async fn rect_view_init() {
        let window = window();
        let state = AppState::new(&window).await;

        let layout = EmptyLayout::new();
        let mut rect = RectView::new("");
        let mut resources = ResourceManager::new();

        rect.init(&layout, &mut resources, &state).unwrap();
    }
}
