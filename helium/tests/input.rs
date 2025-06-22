use winit::event::{DeviceId, ElementState, KeyEvent};
use winit::event::Event::WindowEvent;
use winit::keyboard::{Key, KeyCode, KeyLocation, PhysicalKey};
use crystal::Layout;
use helium::{widgets::Widget,view::View};
use helium_core::GlobalId;

struct MockWidget;

impl Widget for MockWidget{
    fn id(&self) -> GlobalId {
        todo!()
    }

    fn view(&self) -> Box<dyn View> {
        todo!()
    }

    fn layout(&self) -> Box<dyn Layout> {
        todo!()
    }

    fn handle_text_input(&mut self, text: &str) {
        dbg!(text);
    }
}

#[test]
fn handle_input(){
}

