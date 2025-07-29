//! [`Widget`]'s describe what you want to present onto the screen. Agape tries to provide
//! as many [`Widget`]'s as possible for various uses such as [`Text`],[`Button`],[`HStack`]
//! and [`VStack`], and the list goes on. Every widget must implement the [`Widget`] trait.
//!
//! # Creating custom widgets
//! To create a custom widget you can implement the `Widget` trait, it has three required
//! methods:
//! - `id`: Return the widgets [`GlobalId`].
//! - `view`: Return the widgets [`View`] for rendering.
//! - `layout`: Return the widgets [`Layout`] for layout calculations.
//!
//! Additionally, if your widget has any children you will need to implement the `children`
//! method.
mod button;
mod hstack;
mod rect;
mod text;
mod text_field;
mod vstack;

use crate::renderer::{draw_rect, draw_text};
use crate::style::Border;
use agape_core::{Color, GlobalId, Position, Rgba, Size};
use agape_layout::{
    AxisAlignment, BlockLayout, EmptyLayout, HorizontalLayout, IntrinsicSize, Layout, LayoutSolver,
    VerticalLayout,
};
pub use button::Button;
pub use hstack::*;
pub use rect::*;
use std::collections::HashMap;
pub use text::Text;
pub use text_field::TextField;
use tiny_skia::Pixmap;
pub use vstack::*;
use winit::event::ElementState;
use winit::keyboard;

pub trait Widget: WidgetIterator {
    /// Get the `id` of the [`Widget`]
    fn id(&self) -> GlobalId;

    /// Get a [`Widget`] from the widget tree by it's `id`
    fn get(&self, id: GlobalId) -> Option<&dyn Widget> {
        self.iter().find(|&widget| widget.id() == id)
    }

    fn traverse(&self, _f: &mut dyn FnMut(&dyn Widget)) {}
    fn traverse_mut(&mut self, _f: &mut dyn FnMut(&mut dyn Widget)) {}

    /// Get the widgets children.
    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        &mut []
    }

    fn handle_event(&mut self, event: &WidgetEvent) {
        match event {
            WidgetEvent::Hovered(id) => {
                if id == &self.id() {
                    self.hover();
                }
            }
            WidgetEvent::Clicked(id) => {
                if id == &self.id() {
                    self.click();
                }
            }
            WidgetEvent::KeyInput { key, state, text } => {
                self.key_input(key, state, text);
            }
        }

        self.traverse_mut(&mut |child| child.handle_event(event));
    }

    fn click(&mut self) {}
    fn hover(&mut self) {}

    fn build(&self) -> RenderBox {
        todo!()
    }

    fn key_input(&mut self, _key: &keyboard::Key, _state: &ElementState, _text: &Option<String>) {}
}

#[derive(Clone, PartialEq, Debug)]
pub enum WidgetEvent {
    Hovered(GlobalId),
    Clicked(GlobalId),
    KeyInput {
        key: keyboard::Key,
        state: ElementState,
        text: Option<String>,
    },
}

#[derive(Clone, PartialEq, Debug)]
pub struct StateTracker {
    previous_state: HashMap<GlobalId, WidgetState>,
    current_state: HashMap<GlobalId, WidgetState>,
}

// TODO test these
impl StateTracker {
    pub fn new(widget: &dyn Widget) -> Self {
        let mut previous_state = HashMap::new();
        let mut current_state = HashMap::new();

        widget.iter().for_each(|w| {
            let id = w.id();
            previous_state.insert(id, WidgetState::Resting);
            current_state.insert(id, WidgetState::Resting);
        });

        Self {
            previous_state,
            current_state,
        }
    }

    pub fn current_state(&self, id: GlobalId) -> Option<&WidgetState> {
        self.current_state.get(&id)
    }

    pub fn previous_state(&self, id: GlobalId) -> Option<&WidgetState> {
        self.previous_state.get(&id)
    }

    pub fn update_state(&mut self, id: GlobalId, state: WidgetState) {
        let previous_state = self.current_state.get(&id).unwrap();
        self.previous_state.insert(id, *previous_state);
        self.current_state.insert(id, state);
    }
}

/// The current state of the widget
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WidgetState {
    Resting,
    Hovered,
    Clicked,
}

/// An iterator over the [`Widget`] tree.
pub struct WidgetIter<'a> {
    stack: Vec<&'a dyn Widget>,
}

impl<'a> Iterator for WidgetIter<'a> {
    type Item = &'a dyn Widget;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(widget) = self.stack.pop() {
            self.stack.extend(widget.children());
            return Some(widget);
        }
        None
    }
}

pub trait WidgetIterator {
    fn iter(&self) -> WidgetIter<'_>;
}

impl<T: Widget> WidgetIterator for T {
    fn iter(&self) -> WidgetIter<'_> {
        WidgetIter { stack: vec![self] }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct LayoutDescription {
    pub padding: u32,
    pub spacing: u32,
    pub intrinsic_size: IntrinsicSize,
    pub main_axis_alignment: AxisAlignment,
    pub cross_axis_alignment: AxisAlignment,
    pub layout_type: LayoutType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum LayoutType {
    #[default]
    EmptyLayout,
    HorizontalLayout,
    VerticalLayout,
    BlockLayout,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RenderObject {
    Rect {
        border: Option<Border>,
        color: Color<Rgba>,
    },
    Text {
        color: Color<Rgba>,
        content: String,
        font_size: u8,
    },
}

#[derive(Debug)]
pub struct RenderBox {
    id: GlobalId,
    pub size: Size,
    pub position: Position,
    layout_desc: LayoutDescription,
    render_object: RenderObject,
    children: Vec<RenderBox>,
}

impl RenderBox {
    pub fn id(&self) -> GlobalId {
        self.id
    }

    pub fn iter(&self) -> RenderBoxIter<'_> {
        RenderBoxIter { stack: vec![self] }
    }

    /// Update the [`Size`] and [`Position`] of the render box
    /// every frame.
    pub fn solve_layout(&mut self, window_size: Size) {
        let mut layout = self.layout();
        LayoutSolver::solve(&mut *layout, window_size);
        self.update_size(&*layout);
    }

    fn update_size(&mut self, root_layout: &dyn Layout) {
        // TODO don't unwrap, log error instead
        let layout = root_layout.get(self.id).unwrap();
        self.position = layout.position();
        self.size = layout.size();
        self.children
            .iter_mut()
            .for_each(|child| child.update_size(root_layout));
    }

    pub fn layout(&self) -> Box<dyn Layout> {
        // TODO: test this
        match self.layout_desc.layout_type {
            LayoutType::EmptyLayout => Box::new(EmptyLayout {
                id: self.id,
                intrinsic_size: self.layout_desc.intrinsic_size,
                ..Default::default()
            }),
            LayoutType::BlockLayout => {
                let child_layout = self.children[0].layout();
                let mut layout = BlockLayout::new(child_layout);
                layout.id = self.id;
                layout.intrinsic_size = self.layout_desc.intrinsic_size;
                layout.main_axis_alignment = self.layout_desc.main_axis_alignment;
                layout.cross_axis_alignment = self.layout_desc.cross_axis_alignment;
                layout.padding = self.layout_desc.padding;
                Box::new(layout)
            }
            LayoutType::HorizontalLayout => {
                let children = self.children.iter().map(|child| child.layout()).collect();
                let layout = HorizontalLayout {
                    id: self.id,
                    intrinsic_size: self.layout_desc.intrinsic_size,
                    padding: self.layout_desc.padding,
                    spacing: self.layout_desc.spacing,
                    main_axis_alignment: self.layout_desc.main_axis_alignment,
                    cross_axis_alignment: self.layout_desc.cross_axis_alignment,
                    children,
                    ..Default::default()
                };
                Box::new(layout)
            }
            LayoutType::VerticalLayout => {
                let children = self.children.iter().map(|child| child.layout()).collect();
                let layout = VerticalLayout {
                    id: self.id,
                    intrinsic_size: self.layout_desc.intrinsic_size,
                    padding: self.layout_desc.padding,
                    spacing: self.layout_desc.spacing,
                    main_axis_alignment: self.layout_desc.main_axis_alignment,
                    cross_axis_alignment: self.layout_desc.cross_axis_alignment,
                    children,
                    ..Default::default()
                };
                Box::new(layout)
            }
        }
    }

    pub fn render(&self, pixmap: &mut Pixmap) {
        match &self.render_object {
            RenderObject::Rect { border, color } => {
                draw_rect(pixmap, color, self.size, self.position, border.clone());
            }
            RenderObject::Text {
                content, font_size, ..
            } => {
                draw_text(pixmap, content, *font_size as f32, self.position);
            }
        }
        self.children.iter().for_each(|child| child.render(pixmap));
    }
}

pub struct RenderBoxIter<'a> {
    stack: Vec<&'a RenderBox>,
}

impl<'a> Iterator for RenderBoxIter<'a> {
    type Item = &'a RenderBox;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(render_box) = self.stack.pop() {
            self.stack.extend(&render_box.children);
            return Some(render_box);
        }
        None
    }
}
