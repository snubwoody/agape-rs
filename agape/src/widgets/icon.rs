use crate::assets::AssetManager;
use crate::impl_style;
use crate::style::BoxStyle;
use crate::widgets::{Svg, Widget};
use agape_core::{Color, GlobalId};
use agape_layout::{BlockLayout, EmptyLayout, IntrinsicSize, Layout};
use agape_renderer::Renderer;
use agape_renderer::rect::Rect;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::info;
use usvg::Tree;

/// Displays an icon onto the screen.
#[derive(Debug, Clone)]
pub struct Icon {
    id: GlobalId,
    path: PathBuf,
    data: Option<Svg>,
    style: BoxStyle,
}

impl Icon {
    pub fn asset(path: impl AsRef<Path>) -> Self {
        Self {
            id: GlobalId::new(),
            path: path.as_ref().to_path_buf(),
            data: None,
            style: Default::default(),
        }
    }

    impl_style! {}
}

impl Widget for Icon {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn get_assets(&mut self, assets: &AssetManager) {
        // TODO bad!
        if self.data.is_some() {
            return;
        }
        let mut file = assets.get(&self.path).unwrap().unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();
        let mut svg = Svg::bytes(&bytes).unwrap();
        svg.style = self.style.clone();

        self.data = Some(svg);
    }

    fn children(&self) -> Vec<&dyn Widget> {
        match &self.data {
            Some(data) => vec![data],
            None => vec![],
        }
    }

    fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        if let Some(child) = &mut self.data {
            f(child);
            child.traverse(f);
        }
    }

    fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
        if let Some(child) = &self.data {
            let child_layout = child.layout(renderer);
            let mut layout = BlockLayout::new(child_layout);
            layout.id = self.id;
            layout.intrinsic_size = self.style.intrinsic_size;
            return Box::new(layout);
        }

        // FIXME
        let layout = EmptyLayout {
            id: self.id,
            intrinsic_size: self.style.intrinsic_size,
            ..Default::default()
        };
        Box::new(layout)
    }

    fn render(&self, renderer: &mut Renderer, layout_tree: &dyn Layout) {
        let layout = layout_tree.get(self.id).unwrap();
        let size = layout.size();
        let position = layout.position();

        if let Some(child) = &self.data {
            child.render(renderer, layout_tree);
        } else {
            let rect = Rect::new()
                .color(Color::BLACK)
                .size(size.width, size.height)
                .position(position.x, position.y);
            renderer.draw_rect(rect);
        }
    }
}
