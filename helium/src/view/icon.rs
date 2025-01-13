use crate::{app::AppState, geometry::Vertex, resources::ResourceManager, view::View, Color};
use helium_core::color::BLACK;
use std::{collections::HashMap, fmt::Debug};

pub struct IconView {
    id: String,
    img: image::DynamicImage,
    color: Color,
    vertices: Vec<Vertex>,
    resources: HashMap<String, usize>,
}

impl IconView {
    pub fn new(id: &str, img: image::DynamicImage) -> Self {
        Self {
            id: id.to_string(),
            img,
            color: BLACK,
            vertices: vec![],
            resources: HashMap::new(),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl View for IconView {
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
            "Icon Vertex Buffer",
            bytemuck::cast_slice(&vertices),
            &state.device,
        );

        let texture = resources.add_texture("Icon Texture", size, &state.device);
        let texture_view = resources.add_texture_view(texture)?;
        let sampler = resources.add_sampler("Icon Sampler", &state.device);

        let bind_group = resources.add_bind_group(
            "Icon Bind Group",
            &state.context.text_pipeline.texture_bind_group_layout,
            &state.device,
            &[],
            &[texture_view],
            &[sampler],
        )?;

        let img = self
            .img
            .resize(
                size.width as u32,
                size.height as u32,
                image::imageops::FilterType::CatmullRom,
            )
            .to_rgba8();

        resources.write_texture(texture, size, &img, &state.queue)?;

        self.vertices = vertices;
        self.resources.insert("Bind group".to_string(), bind_group);
        self.resources
            .insert("Vertex buffer".to_string(), vertex_buffer);

        Ok(())
    }

	fn resize(&mut self, layout: &dyn crystal::Layout, resources: &ResourceManager, state: &AppState) {
		
	}

    fn draw(
        &mut self,
        pass: &mut wgpu::RenderPass,
        resources: &ResourceManager,
        context: &crate::geometry::RenderContext,
        state: &AppState,
    ) {
        let bind_group = resources
            .bind_group(*self.resources.get("Bind group").unwrap())
            .unwrap();
        let vertex_buffer = resources
            .buffer(*self.resources.get("Vertex buffer").unwrap())
            .unwrap();

        pass.set_pipeline(&context.icon_pipeline.pipeline);
        pass.set_bind_group(0, &context.icon_pipeline.window_bind_group, &[]);
        pass.set_bind_group(1, &bind_group, &[]);
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        pass.draw(0..self.vertices.len() as u32, 0..1);
    }
}

impl Debug for IconView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IconSurface")
            .field("id", &self.id)
            .field("color", &self.color)
            .finish()
    }
}
