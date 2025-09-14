use crate::assets::AssetManager;
use crate::impl_style;
use crate::style::BoxStyle;
use crate::widgets::{Svg, Widget};
use agape_core::GlobalId;
use agape_layout::{EmptyLayout, IntrinsicSize, Layout};
use agape_renderer::Renderer;
use std::fs;
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
        let file = assets.get("PARTYNEXTDOOR Album Cover.jpg").unwrap();
        dbg!(file);
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

    fn layout(&self, _: &mut Renderer) -> Box<dyn Layout> {
        // FIXME
        let layout = EmptyLayout {
            id: self.id,
            intrinsic_size: self.style.intrinsic_size,
            ..Default::default()
        };
        Box::new(layout)
    }

    fn render(&self, renderer: &mut Renderer, layout: &dyn Layout) {
        if let Some(child) = &self.data {
            child.render(renderer, layout);
        }
    }
}
