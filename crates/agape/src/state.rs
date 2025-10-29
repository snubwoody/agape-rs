use crate::MessageQueue;
use crate::assets::AssetManager;
use crate::message::MouseButtonDown;
use crate::resources::CursorPosition;
use crate::widgets::{View, Widget};
use agape_core::{Position, Size};
use agape_layout::{Layout, solve_layout};
use agape_renderer::Renderer;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::NamedKey;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Scroll(pub f32);

// TODO: Add method to trigger rebuild
pub struct State<T> {
    cursor_position: CursorPosition,
    message_queue: MessageQueue,
    layout: Box<dyn Layout>,
    widget: Box<dyn Widget>,
    asset_manager: AssetManager,
    window_size: Size,
    renderer: Renderer,
    context: Context,
    root: Box<dyn View<Widget = T>>,
}

impl<T> State<T>
where
    T: Widget + 'static,
{
    pub fn new(root: impl View<Widget = T> + 'static) -> Self {
        let mut context = Context::new();
        let mut renderer = Renderer::new();
        let widget = root.view(&mut context);
        let layout = widget.layout(&mut renderer);
        Self {
            asset_manager: AssetManager::new("."),
            message_queue: MessageQueue::default(),
            cursor_position: CursorPosition::default(),
            window_size: Size::default(),
            widget: Box::new(widget),
            layout,
            context,
            root: Box::new(root),
            renderer,
        }
    }

    pub fn asset_dir(&mut self, path: impl AsRef<Path>) {
        self.asset_manager = AssetManager::new(path);
    }

    pub fn update(&mut self) {
        self.widget = Box::new(self.root.view(&mut self.context));
        self.message_queue.tick();
        self.message_queue.clear();
        // Assets need to be fetched before recreating the
        // layout tree
        self.widget.tick(&mut self.message_queue);
        self.widget.get_assets(&self.asset_manager);
        self.widget.traverse(&mut |widget| {
            widget.tick(&mut self.message_queue);
            widget.get_assets(&self.asset_manager);
        });
        let mut layout = self.widget.layout(&mut self.renderer);
        solve_layout(layout.as_mut(), self.window_size);
        self.layout = layout;
        self.check_hovered();
        self.check_clicked();
    }

    pub fn render(&mut self) {
        self.renderer.pixmap_mut().fill(tiny_skia::Color::WHITE);
        self.widget.render(&mut self.renderer, self.layout.as_ref());
    }

    /// Get a reference to the [`Renderer`].
    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    /// Get a mutable reference to the [`Renderer`].
    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    pub fn messages_mut(&mut self) -> &mut MessageQueue {
        &mut self.message_queue
    }

    pub fn resize(&mut self, size: Size) {
        self.window_size = size;
        self.renderer.resize(size.width as u32, size.height as u32);
    }

    pub fn update_cursor_position(&mut self, position: Position) {
        self.cursor_position.update(position);
    }

    pub fn check_hovered(&mut self) {
        let widget = &mut self.widget;
        let layout = &self.layout;
        if let Some(l) = layout.get(widget.id())
            && self.cursor_position.mouse_entered(l)
        {
            widget.hover(&mut self.message_queue);
            widget.mouse_entered(&mut self.message_queue);
        }

        if let Some(l) = layout.get(widget.id())
            && self.cursor_position.mouse_left(l)
        {
            widget.hover(&mut self.message_queue);
            widget.mouse_left(&mut self.message_queue);
        }
        widget.traverse(&mut |widget| {
            if let Some(l) = layout.get(widget.id())
                && self.cursor_position.mouse_entered(l)
            {
                widget.mouse_entered(&mut self.message_queue);
            }

            if let Some(l) = layout.get(widget.id())
                && self.cursor_position.mouse_left(l)
            {
                widget.mouse_left(&mut self.message_queue);
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

    pub fn key_event(&mut self, event: &KeyEvent) {
        if let ElementState::Released = event.state {
            return;
        }
        if let Some(named_key) = NamedKeyInput::from_key(&event.logical_key) {
            self.message_queue.add(named_key);
        }

        if let Some(character) = CharacterInput::from_key(&event.logical_key) {
            self.message_queue.add(character);
        }
    }
}

#[derive(Clone)]
pub struct StateCell<T> {
    data: Arc<Mutex<T>>,
}

impl<T: Default> Default for StateCell<T> {
    fn default() -> Self {
        Self {
            data: Arc::new(Mutex::new(T::default())),
        }
    }
}

impl<T> StateCell<T>
where
    T: Clone,
{
    pub fn new(data: T) -> Self {
        Self {
            data: Arc::new(Mutex::new(data)),
        }
    }

    pub fn get(&self) -> T {
        self.data.lock().unwrap().clone()
    }

    pub fn set(&self, mut f: impl FnMut(T) -> T) {
        *self.data.lock().unwrap() = f(self.get());
    }

    pub fn update(&self, mut f: impl FnMut(&mut T)) {
        f(&mut *self.data.lock().unwrap());
    }
}

#[derive(Debug, Default)]
pub struct Context {
    items: HashMap<TypeId, Box<dyn Any>>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<T: Clone + 'static>(&mut self, item: T) {
        self.items
            .insert(item.type_id(), Box::new(StateCell::new(item)));
    }

    pub fn get<T: Clone + 'static>(&self) -> StateCell<T> {
        self.items
            .get(&TypeId::of::<T>())
            .and_then(|item| item.downcast_ref::<StateCell<T>>())
            .cloned()
            .unwrap()
    }

    pub fn get_or_init<T: Clone + 'static>(&mut self, f: impl FnOnce() -> T) -> StateCell<T> {
        match self.items.get(&TypeId::of::<T>()) {
            Some(item) => item.downcast_ref::<StateCell<T>>().cloned().unwrap(),
            None => {
                self.insert(f());
                self.get::<T>()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NamedKeyInput(pub NamedKey);
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct CharacterInput(pub String);

impl NamedKeyInput {
    pub fn from_key(key: &winit::keyboard::Key) -> Option<Self> {
        if let winit::keyboard::Key::Named(key) = key {
            return Some(Self(*key));
        }
        None
    }
}

impl CharacterInput {
    pub fn from_key(key: &winit::keyboard::Key) -> Option<Self> {
        if let winit::keyboard::Key::Character(chr) = key {
            return Some(Self(chr.to_string()));
        }
        None
    }
}
