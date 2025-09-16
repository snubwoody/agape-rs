use crate::assets::AssetManager;
use crate::message::MouseButtonDown;
use crate::resources::{CursorPosition, EventQueue};
use crate::widgets::{View, ViewTree, Widget, WidgetTree};
use crate::{LayoutTree, MessageQueue, WindowSize};
use agape_core::{Position, Size};
use agape_layout::{Layout, solve_layout};
use agape_renderer::Renderer;
use std::path::Path;

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

    pub fn asset_dir(&mut self, path: impl AsRef<Path>) {
        self.asset_manager = AssetManager::new(path);
    }

    pub fn update(&mut self) {
        self.view.update(&mut self.message_queue);
        self.widget = self.view.view();
        let mut layout = self.widget.layout(&mut self.renderer);
        solve_layout(layout.as_mut(), self.window_size);
        self.layout = layout;
        self.check_hovered();
        self.check_clicked();
        // TODO test this please
        self.message_queue.tick();
        self.message_queue.clear();
        // FIXME
        // self.widget.get_assets(&self.asset_manager);
        // self.widget.traverse(&mut |w| {
        //     w.get_assets(&self.asset_manager);
        // });
    }

    pub fn render(&mut self) {
        self.renderer.pixmap_mut().fill(tiny_skia::Color::WHITE);
        self.widget.render(&mut self.renderer, self.layout.as_ref());
    }

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    pub fn messages_mut(&mut self) -> &mut MessageQueue {
        &mut self.message_queue
    }

    pub fn resize(&mut self, size: Size) {
        self.window_size = size;
        self.renderer.resize(size.width as u32, size.height as u32);
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

    pub fn check_hovered(&mut self) {
        let widget = &mut self.widget;
        let layout = &self.layout;
        if let Some(l) = layout.get(widget.id())
            && self.cursor_position.just_hovered(l)
        {
            widget.hover(&mut self.message_queue);
        }
        widget.traverse(&mut |widget| {
            if let Some(l) = layout.get(widget.id())
                && self.cursor_position.just_hovered(l)
            {
                widget.hover(&mut self.message_queue);
            }
        });
    }

    pub fn check_clicked(&mut self) {
        if !self.message_queue.has::<MouseButtonDown>() {
            return;
        }

        let widget = self.widget.as_mut();
        let layout = self.layout.as_ref();
        if let Some(l) = layout.get(widget.id())
            && self.cursor_position.is_hovered(l)
        {
            widget.click(&mut self.message_queue);
        }
        widget.traverse(&mut |widget| {
            if let Some(l) = layout.get(widget.id())
                && self.cursor_position.is_hovered(l)
            {
                widget.click(&mut self.message_queue);
            }
        });
    }
}
