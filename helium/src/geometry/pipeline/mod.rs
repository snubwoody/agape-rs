pub mod image;
pub mod text;
pub mod rect;
pub mod circle;
use std::str;
use circle::CirclePipeline;
use rect::RectPipeline;
use text::TextPipeline;
use wgpu::{
	ColorTargetState, 
	FragmentState, 
	MultisampleState, 
	PipelineLayoutDescriptor, 
	PrimitiveState, 
	RenderPipelineDescriptor,
	VertexBufferLayout, VertexState
};
use helium_core::size::Size;
use super::uniform::{Uniform, UniformBuilder};

struct RenderPipelineBuilder<'a>{
	label:String,
	shader:&'a wgpu::ShaderModule,
	vertex_entry_point:String,
	fragment_entry_point:String,
	layout:Option<&'a wgpu::PipelineLayout>,
	bind_group_layouts:Vec<&'a wgpu::BindGroupLayout>,
	buffers:Vec<VertexBufferLayout<'a>>
}

impl<'a> RenderPipelineBuilder<'a> {
	fn new(label:&str,shader:&'a wgpu::ShaderModule) -> Self{
		let vertex_entry_point = String::from("vs_main");
		let fragment_entry_point = String::from("fs_main");
		
		Self{
			label:label.to_owned(),
			shader,
			vertex_entry_point,
			fragment_entry_point,
			buffers:vec![],
			bind_group_layouts:vec![],
			layout:None
		}
	}


	fn vertex_entry_point(mut self,entry_point:&str) -> Self {
		self.vertex_entry_point = entry_point.to_owned();
		self
	}

	fn fragment_entry_point(mut self,entry_point:&str) -> Self {
		self.fragment_entry_point = entry_point.to_owned();
		self
	}

	fn add_bind_group_layout(mut self,layout:&'a wgpu::BindGroupLayout) -> Self{
		self.bind_group_layouts.push(layout);
		self
	}

	fn layout(mut self,layout:&'a wgpu::PipelineLayout) -> Self{
		self.layout = Some(layout);
		self
	}

	fn add_buffer(mut self,buffer:wgpu::VertexBufferLayout<'a>) -> Self{
		self.buffers.push(buffer);
		self
	}

	fn build(self,device:&wgpu::Device,config:&wgpu::SurfaceConfiguration) -> wgpu::RenderPipeline{
		let render_pipeline_layout = 
			device.create_pipeline_layout(
				&PipelineLayoutDescriptor{
					label: Some(format!("{} Pipeline Layout",self.label).as_str()),
					bind_group_layouts: self.bind_group_layouts.as_slice(),
					push_constant_ranges: &[]
				}
			);

		device.create_render_pipeline(
			&RenderPipelineDescriptor { 
				label: Some(format!("{} Pipeline Layout",self.label).as_str()), 
				layout: Some(&render_pipeline_layout), 
				vertex: VertexState{
					module: &self.shader,
					entry_point: &self.vertex_entry_point,
					compilation_options: Default::default(),
					buffers: &self.buffers
				}, 
				fragment: Some(FragmentState{
					module: &self.shader,
					entry_point: &self.fragment_entry_point,
					compilation_options: Default::default(),
					targets: &[Some(ColorTargetState {
						format: config.format,
						blend: Some(wgpu::BlendState::ALPHA_BLENDING), // TODO check pre-multiplied alpha blending
						write_mask: wgpu::ColorWrites::ALL,
					})]
				}), 
				primitive: PrimitiveState{
					topology: wgpu::PrimitiveTopology::TriangleList,
                	strip_index_format: None,
                	front_face: wgpu::FrontFace::Ccw,
                	cull_mode: None,
                	unclipped_depth: false,
                	polygon_mode: wgpu::PolygonMode::Fill,
                	conservative: false,
				}, 
				multisample: MultisampleState {
					count: 1,
					mask: !0,
					alpha_to_coverage_enabled: false,
				}, 
				depth_stencil: None, 
				multiview: None, 
				cache: None 
			}
		)
	}
}

/// Contains the renderers
pub struct RenderContext {
	pub rect_renderer: RectPipeline,
	pub text_renderer: TextPipeline,
	pub circle_renderer: CirclePipeline,
	pub window_uniform:Uniform
}

impl RenderContext {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, size: &Size) -> Self {
		let rect_renderer = RectPipeline::new(device, config, size);
		let text_renderer = TextPipeline::new(device, config, size);
		let circle_renderer = CirclePipeline::new(device, config, size);

		let window_buffer = UniformBuilder::new()
			.label("Window uniform")
			.contents(&[size.width,size.height])
			.build(device);
		
        Self {
			rect_renderer,
			text_renderer,
			circle_renderer,
			window_uniform:window_buffer
        }
    }
}