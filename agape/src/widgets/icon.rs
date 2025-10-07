use crate::assets::AssetManager;
use crate::element::{Element, ElementKind, LayoutKind};
use crate::impl_style;
use crate::style::BoxStyle;
use crate::widgets::{Svg, Widget};
use agape_core::{Color, GlobalId};
use agape_layout::{BlockLayout, EmptyLayout, Layout};
use agape_renderer::Renderer;
use agape_renderer::rect::Rect;
use std::io::Read;
use std::path::{Path, PathBuf};

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

    fn build(&self) -> Element {
        if let Some(data) = &self.data {
            let element = data.build();
            let kind = ElementKind::Rect {
                style: self.style.clone(),
                layout: LayoutKind::Block,
            };

            return Element {
                id: self.id,
                kind,
                children: vec![element],
                on_click: None,
            };
        }

        let kind = ElementKind::Rect {
            style: self.style.clone(),
            layout: LayoutKind::Empty,
        };

        Element {
            id: self.id,
            kind,
            children: Vec::new(),
            on_click: None,
        }
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
}
