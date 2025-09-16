use crate::assets::AssetManager;
use crate::resources::{CursorPosition, EventQueue};
use crate::widgets::{View, ViewTree, Widget, WidgetTree};
use crate::{LayoutTree, MessageQueue, WindowSize};
use agape_core::{Position, Size};
use agape_layout::Layout;
use agape_renderer::Renderer;

pub struct State {
    cursor_position: CursorPosition,
    event_queue: EventQueue,
    message_queue: MessageQueue,
    view: Box<dyn View>,
    layout: Box<dyn Layout>,
    widget: Box<dyn Widget>,
    asset_manager: AssetManager,
    window_size: Size,
    renderer: Renderer,
}

impl State {
    pub fn new(view: impl View + 'static) -> Self {
        let mut renderer = Renderer::new();
        let widget = view.view();
        let layout = widget.layout(&mut renderer);

        Self {
            asset_manager: AssetManager::new("."),
            event_queue: EventQueue::default(),
            message_queue: MessageQueue::default(),
            view: Box::new(view),
            cursor_position: CursorPosition::default(),
            window_size: Size::default(),
            widget,
            layout,
            renderer,
        }
    }

    pub fn update(&mut self) {
        self.view.update(&mut self.message_queue);
        self.widget = self.view.view();
        // TODO layout
    }

    pub fn render(&mut self) {
        self.renderer.pixmap_mut().fill(tiny_skia::Color::WHITE);
        self.widget.render(&mut self.renderer, self.layout.as_ref());
    }

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    pub fn resize(&mut self, size: Size) {
        self.window_size = size;
        self.renderer.resize(size.width as u32, size.height as u32);
    }

    pub fn update_window_size(&mut self, size: Size) {
        self.window_size = size;
    }

    pub fn window_size(&self) -> Size {
        self.window_size
    }

    pub fn update_cursor_position(&mut self, position: Position) {
        self.cursor_position.update(position);
    }

    pub fn cursor_position(&self) -> CursorPosition {
        self.cursor_position
    }
}
