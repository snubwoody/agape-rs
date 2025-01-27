use crate::events::{EventContext, EventManager};
use crate::widgets::Widget;
use crystal::{LayoutSolver, Size};
use helium_renderer::Renderer;

pub struct Page {
    layout: Box<dyn crystal::Layout>,
    widget: Box<dyn Widget>,
    events: EventManager,
}

impl Page {
    pub fn new(cx: EventContext, widget: impl Widget + 'static) -> Self {
        Self {
            layout: widget.layout(),
            events: EventManager::new(cx, &*widget.layout()),
            widget: Box::new(widget),
        }
    }

    pub fn handle(&mut self, event: &winit::event::WindowEvent) {
        self.events.process(event, &*self.layout);
        self.widget.tick(self.events.elements());
    }

    pub fn resize(&mut self, size:Size) {
        LayoutSolver::solve(&mut *self.layout,size);
    }

	pub fn draw(&self, renderer:&mut Renderer){
		self.widget.iter().for_each(|w|{
			if let Some(layout) = self.layout.get(w.id()){
				// TODO add an error or similar here; every widget should have a layout
				w.draw(layout,renderer); 
			}
		});
	}
}
