use crate::{
    app::AppState, error::Error, geometry::Vertex, resources::ResourceManager, view::View,
};
use crystal::Layout;
use helium_core::color::WHITE;
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone, PartialEq)]
pub struct ImageView {
    id: String,
    image: ::image::DynamicImage,
    vertices: Vec<Vertex>,
    resources: HashMap<String, usize>,
}

impl ImageView {
    pub fn new(id: &str, image: ::image::DynamicImage) -> Self {
        Self {
            id: id.to_string(),
            image,
            vertices: vec![],
            resources: HashMap::new(),
        }
    }
}

impl View for ImageView {
    fn id(&self) -> &str {
        &self.id
    }

    fn init(
        &mut self,
        layout: &dyn Layout,
        resources: &mut ResourceManager,
        state: &AppState,
    ) -> Result<(), Error> {
        let size = layout.size();
        let position = layout.position();

        let vertices = Vertex::quad(size, position, WHITE);

        let vertex_buffer = resources.add_vertex_buffer_init(
            "Image Vertex Buffer",
            bytemuck::cast_slice(&vertices),
            &state.device,
        );

        let texture = resources.add_texture("Image texture", size, &state.device);

        let texture_view = resources.add_texture_view(texture)?;
        let sampler = resources.add_sampler("Image texture sampler", &state.device);

        let image = self
            .image
            .resize(
                size.width as u32,
                size.height as u32,
                image::imageops::FilterType::Nearest, // This is by far the fastest filter type
            )
            .to_rgba8();

        resources.write_texture(texture, size, &image, &state.queue)?;

        let bind_group = resources.add_bind_group(
            "Image texture bind group",
            &state.context.image_pipeline.texture_bind_group_layout,
            &state.device,
            &[],
            &[texture_view],
            &[sampler],
        )?;

        self.vertices = vertices;
        self.resources.insert("Bind group".to_string(), bind_group);
        self.resources
            .insert("Vertex buffer".to_string(), vertex_buffer);

        Ok(())
    }

	fn resize(
		&mut self, 
		layout: &dyn crystal::Layout, 
		resources: &ResourceManager, 
		state: &AppState
	) -> Result<(),crate::Error> {
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

        pass.set_pipeline(&context.image_pipeline.pipeline);
        pass.set_bind_group(0, &context.image_pipeline.window_bind_group, &[]);
        pass.set_bind_group(1, bind_group, &[]);
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        pass.draw(0..self.vertices.len() as u32, 0..1);
    }
}
