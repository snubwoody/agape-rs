use helium_core::Color;
use crate::{resources::ResourceManager, vertex::Vertex};
use std::collections::HashMap;

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

impl RectView {
    fn id(&self) -> &str {
        &self.id
    }

    fn init(
        &mut self,
        layout: &dyn crystal::Layout,
        resources: &mut ResourceManager,
        device:&wgpu::Device,
    ) {
        // let size = layout.size();
        // let position = layout.position();

        // let vertices = Vertex::quad(size, position, self.color);

        // let vertex_buffer = resources.add_vertex_buffer_init(
        //     "Rect Vertex Buffer",
        //     bytemuck::cast_slice(&vertices),
        //     device,
        // );

        // let size_buffer = resources.add_uniform_init(
        //     "Rect Size Buffer",
        //     bytemuck::cast_slice(&[size.width, size.height]),
        //     device,
        // );

        // let position_buffer = resources.add_uniform_init(
        //     "Rect Position Buffer",
        //     bytemuck::cast_slice(&[position.x, position.y]),
        //     device,
        // );

        // let radius_buffer = resources.add_uniform_init(
        //     "Rect Corner Radius Buffer",
        //     bytemuck::cast_slice(&[self.corner_radius]),
        //     device,
        // );

        // let bind_group = resources.add_bind_group(
        //     "Rect Bind Group",
        //     context.rect_pipeline.bounds_layout,
        //     device,
        //     &[radius_buffer, size_buffer, position_buffer],
        //     &[],
        //     &[],
        // )?;

        // self.resources
        //     .insert("Vertex buffer".to_string(), vertex_buffer);
        // self.resources.insert("Size".to_string(), size_buffer);
        // self.resources
        //     .insert("Position".to_string(), position_buffer);
        // self.resources.insert("Size".to_string(), size_buffer);
        // self.resources
        //     .insert("Corner radius".to_string(), radius_buffer);
        // self.resources.insert("Bind group".to_string(), bind_group);
        // self.vertices = vertices;

        // Ok(())
    }

    fn resize(
        &mut self,
        layout: &dyn crystal::Layout,
        resources: &ResourceManager,
		queue:&wgpu::Queue,
    ) -> Result<(), crate::Error> {
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
            queue,
        )?;
        resources.write_buffer(
            *position_buffer,
            0,
            bytemuck::cast_slice(&[position.x, position.y]),
            queue,
        )?;
        resources.write_buffer(
            *size_buffer,
            0,
            bytemuck::cast_slice(&[size.width, size.height]),
            queue,
        )?;

        Ok(())
    }

    fn draw(
        &mut self,
        pass: &mut wgpu::RenderPass,
        resources: &ResourceManager,
		pipeline:&wgpu::RenderPipeline
    ) {
        // let vertex_buffer = resources
        //     .buffer(*self.resources.get("Vertex buffer").unwrap())
        //     .unwrap();
        // let bind_group = resources
        //     .bind_group(*self.resources.get("Bind group").unwrap())
        //     .unwrap();

        // pass.set_pipeline(pipeline);
        // pass.set_bind_group(0, window_bind_group, &[]);
        // pass.set_bind_group(1, bind_group, &[]);
        // pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        // pass.draw(0..self.vertices.len() as u32, 0..1);
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

        let layout = EmptyLayout::new();
        let mut rect = RectView::new("");
        let mut resources = ResourceManager::new();

        //rect.init(&layout, &mut resources, &state).unwrap();
    }
}