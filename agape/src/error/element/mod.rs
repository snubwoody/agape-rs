use crate::style::BoxStyle;
use crate::widgets::Widget;
use agape_core::GlobalId;
use agape_layout::{EmptyLayout, HorizontalLayout, Layout};
use agape_renderer::Renderer;
use agape_renderer::rect::Rect;

pub trait Element {
    /// Get the `id` of the [`Widget`].
    fn id(&self) -> GlobalId;

    /// Construct a [`Layout`] to solve layout for the whole
    /// widget tree.
    fn layout(&self, _: &mut Renderer) -> Box<dyn Layout>;

    /// Draw the widget to the screen.
    fn render(&self, _: &mut Renderer, _: &dyn Layout);
}

pub struct BoxElement {
    id: GlobalId,
    style: BoxStyle,
}

impl Element for BoxElement {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn layout(&self, _: &mut Renderer) -> Box<dyn Layout> {
        let layout = EmptyLayout {
            id: self.id,
            intrinsic_size: self.style.intrinsic_size,
            ..Default::default()
        };
        Box::new(layout)
    }

    fn render(&self, renderer: &mut Renderer, layout: &dyn Layout) {
        let layout = layout.get(self.id).unwrap();
        let size = layout.size();
        let position = layout.position();

        let mut rect = agape_renderer::rect::Rect::new()
            .size(size.width, size.height)
            .position(position.x, position.y)
            .corner_radius(self.style.corner_radius)
            .color(self.style.background_color.clone());

        rect.border = self.style.border.clone();
        renderer.draw_rect(rect);
    }
}

pub struct TextElement {}
pub struct ColumnElement {}

pub struct RowElement {
    id: GlobalId,
    style: BoxStyle,
    children: Vec<Box<dyn Element>>,
}

impl Element for RowElement {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
        let children: Vec<Box<dyn Layout>> =
            self.children.iter().map(|w| w.layout(renderer)).collect();
        let layout = HorizontalLayout {
            id: self.id,
            intrinsic_size: self.style.intrinsic_size,
            children,
            ..Default::default()
        };

        Box::new(layout)
    }

    fn render(&self, renderer: &mut Renderer, layout: &dyn Layout) {
        let layout = layout.get(self.id).unwrap();
        let size = layout.size();
        let position = layout.position();
        let mut rect = Rect::new()
            .size(size.width, size.height)
            .color(self.style.background_color.clone())
            .position(position.x, position.y)
            .corner_radius(self.style.corner_radius);

        rect.border = self.style.border.clone();
        renderer.draw_rect(rect);
        // TODO: test this
        self.children
            .iter()
            .for_each(|child| child.render(renderer, layout));
    }
}
