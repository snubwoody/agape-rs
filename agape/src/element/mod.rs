use crate::MessageQueue;
use crate::message::MouseButtonDown;
use crate::resources::CursorPosition;
use crate::style::BoxStyle;
use agape_core::GlobalId;
use agape_layout::{
    BlockLayout, EmptyLayout, HorizontalLayout, IntrinsicSize, Layout, VerticalLayout,
};
use agape_renderer::Renderer;
use agape_renderer::rect::Rect;
use image::DynamicImage;
use std::sync::Arc;
use usvg::Tree;
// TODO: State arena
// TODO: impl Debug manually
pub struct Element {
    pub id: GlobalId,
    pub kind: ElementKind,
    pub children: Vec<Element>,
    /// Label used for debugging
    pub label: String,
    pub on_click: Option<fn(&mut MessageQueue)>,
}

impl Element {
    pub(crate) fn check_clicked(
        &mut self,
        message_queue: &mut MessageQueue,
        layout: &dyn Layout,
        cursor_position: CursorPosition,
    ) {
        if !message_queue.has::<MouseButtonDown>() || self.on_click.is_some() {
            return;
        }

        if let Some(l) = layout.get(self.id)
            && cursor_position.is_hovered(l)
            && let Some(f) = self.on_click
        {
            f(message_queue);
            // self.on_click.as_mut().unwrap()(message_queue);
        }
        self.children
            .iter_mut()
            .for_each(|child| child.check_clicked(message_queue, layout, cursor_position));
    }

    pub(crate) fn check_hovered(&self) {}

    fn rect_layout(
        &self,
        renderer: &mut Renderer,
        style: &BoxStyle,
        kind: &LayoutKind,
    ) -> Box<dyn Layout> {
        let layout: Box<dyn Layout> = match kind {
            LayoutKind::Empty => {
                let layout = EmptyLayout {
                    id: self.id,
                    intrinsic_size: style.intrinsic_size,
                    ..Default::default()
                };
                Box::new(layout)
            }
            LayoutKind::Block => {
                let child = self.children[0].layout(renderer);
                let mut layout = BlockLayout::new(child);
                layout.id = self.id;
                layout.padding = style.padding;
                layout.intrinsic_size = style.intrinsic_size;
                Box::new(layout)
            }
            LayoutKind::Horizontal => {
                let children: Vec<Box<dyn Layout>> =
                    self.children.iter().map(|w| w.layout(renderer)).collect();
                let layout = HorizontalLayout {
                    id: self.id,
                    intrinsic_size: style.intrinsic_size,
                    main_axis_alignment: style.main_axis_alignment,
                    cross_axis_alignment: style.cross_axis_alignment,
                    spacing: style.spacing,
                    padding: style.padding,
                    children,
                    ..Default::default()
                };

                Box::new(layout)
            }
            LayoutKind::Vertical => {
                let children: Vec<Box<dyn Layout>> =
                    self.children.iter().map(|w| w.layout(renderer)).collect();
                let layout = VerticalLayout {
                    id: self.id,
                    intrinsic_size: style.intrinsic_size,
                    main_axis_alignment: style.main_axis_alignment,
                    cross_axis_alignment: style.cross_axis_alignment,
                    spacing: style.spacing,
                    padding: style.padding,
                    children,
                    ..Default::default()
                };

                Box::new(layout)
            }
        };
        layout
    }

    pub fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
        match &self.kind {
            ElementKind::Text { font_size, value } => {
                let size = renderer.text_size(&value, *font_size);
                let layout = EmptyLayout {
                    id: self.id,
                    intrinsic_size: IntrinsicSize::from(size),
                    ..Default::default()
                };
                Box::new(layout)
            }
            ElementKind::Rect { style, layout } => self.rect_layout(renderer, style, layout),
            ElementKind::Image { style, .. } => {
                let layout = EmptyLayout {
                    id: self.id,
                    intrinsic_size: style.intrinsic_size,
                    ..Default::default()
                };

                Box::new(layout)
            }
            ElementKind::Svg { style, .. } => {
                let layout = EmptyLayout {
                    id: self.id,
                    intrinsic_size: style.intrinsic_size,
                    ..Default::default()
                };
                Box::new(layout)
            }
        }
    }

    pub fn render(&self, renderer: &mut Renderer, layout: &dyn Layout) {
        match &self.kind {
            ElementKind::Text { font_size, value } => {
                if value.trim().is_empty() {
                    return;
                }

                let layout = layout.get(self.id).unwrap();
                let position = layout.position();
                let mut text = agape_renderer::Text::new(value.as_str());
                text.font_size = *font_size;
                text.position = position;
                renderer.draw_text(text)
            }
            ElementKind::Rect { style, .. } => {
                let layout = layout.get(self.id).unwrap();
                let size = layout.size();
                let position = layout.position();

                let mut rect = Rect::new()
                    .size(size.width, size.height)
                    .position(position.x, position.y)
                    .corner_radius(style.corner_radius)
                    .color(style.background_color.clone());

                rect.border = style.border.clone();
                renderer.draw_rect(rect);
            }
            ElementKind::Image { data, .. } => {
                let layout = layout.get(self.id).unwrap();
                let size = layout.size();
                let position = layout.position();
                let mut image = agape_renderer::image::Image::new(data.clone());
                image.size = size;
                image.position = position;
                renderer.draw_image(image);
            }
            ElementKind::Svg { data, .. } => {
                let layout = layout.get(self.id).unwrap();
                let size = layout.size();
                let position = layout.position();
                let mut svg = agape_renderer::Svg::new(data.clone());
                svg.size = size;
                svg.position = position;
                renderer.draw_svg(svg);
            }
        }
        self.children
            .iter()
            .for_each(|child| child.render(renderer, layout));
    }
}

#[derive(Debug)]
pub enum ElementKind {
    Text {
        font_size: f32,
        value: String,
    },
    Rect {
        style: BoxStyle,
        layout: LayoutKind,
    },
    Image {
        data: Arc<DynamicImage>,
        style: BoxStyle,
    },
    Svg {
        data: Arc<Tree>,
        style: BoxStyle,
    },
}

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub enum LayoutKind {
    Empty,
    Block,
    Vertical,
    Horizontal,
}
