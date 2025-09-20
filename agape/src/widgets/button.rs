use super::{Callback, Widget};
use crate::style::BoxStyle;
use crate::{MessageQueue, impl_style};
use agape_core::GlobalId;
use agape_layout::{BlockLayout, Layout};
use agape_renderer::Renderer;
use agape_renderer::rect::Rect;
use tracing::info;

// TODO: add prefix and suffix icon
/// A widget that wraps another widget.
pub struct Button<W> {
    id: GlobalId,
    child: W,
    style: BoxStyle,
    padding: u32,
    hover_callback: Option<Callback>,
    click_callback: Option<Callback>,
}

impl<W> Button<W> {
    pub fn new(child: W) -> Self {
        Self {
            id: GlobalId::new(),
            style: BoxStyle::new(),
            child,
            padding: 0,
            hover_callback: None,
            click_callback: None,
        }
    }

    pub fn padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    pub fn on_hover(mut self, f: impl FnMut(&mut MessageQueue) + Send + Sync + 'static) -> Self {
        self.hover_callback = Some(Box::new(f));
        self
    }

    pub fn on_click(mut self, f: impl FnMut(&mut MessageQueue) + Send + Sync + 'static) -> Self {
        self.click_callback = Some(Box::new(f));
        self
    }

    impl_style!();
}

impl<W: Widget> Widget for Button<W> {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        f(&mut self.child);
        self.child.traverse(f);
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![&self.child]
    }

    fn click(&mut self, messages: &mut MessageQueue) {
        if let Some(f) = &mut self.click_callback {
            f(messages);
        }
    }

    fn hover(&mut self, messages: &mut MessageQueue) {
        if let Some(f) = &mut self.hover_callback {
            f(messages);
        }
    }

    fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
        let child = self.child.layout(renderer);
        let mut layout = BlockLayout::new(child);
        layout.id = self.id;
        layout.padding = self.padding;
        layout.intrinsic_size = self.style.intrinsic_size;
        Box::new(layout)
    }

    fn render(&self, renderer: &mut Renderer, layout: &dyn Layout) {
        let layout = layout.get(self.id).unwrap();
        let size = layout.size();
        let position = layout.position();
        let mut rect = Rect::new()
            .size(size.width, size.height)
            .position(position.x, position.y)
            .corner_radius(self.style.corner_radius)
            .color(self.style.background_color.clone());

        rect.border = self.style.border.clone();
        renderer.draw_rect(rect);
        self.child.render(renderer, layout);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::widgets::Rect;
    use agape_core::{Color, Size};
    use agape_layout::solve_layout;

    #[test]
    fn traverse() {
        let mut button = Button::new(Rect::default());
        let mut ids = vec![];
        button.traverse(&mut |w| {
            ids.push(w.id());
        });
        assert_eq!(ids.len(), 1);
    }

    #[test]
    fn render_child() {
        let button = Button::new(
            Rect::new()
                .fixed(100.0, 100.0)
                .background_color(Color::rgb(53, 102, 145)),
        )
        .fixed(100.0, 100.0);

        let mut renderer = Renderer::new();
        renderer.resize(100, 100);
        let mut layout = button.layout(&mut renderer);
        solve_layout(layout.as_mut(), Size::default());
        button.render(&mut renderer, layout.as_ref());

        for pixel in renderer.pixmap().pixels() {
            assert_eq!(pixel.red(), 53);
            assert_eq!(pixel.green(), 102);
            assert_eq!(pixel.blue(), 145);
        }
    }
}
