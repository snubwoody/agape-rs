use crate::widgets::{LayoutDescription, RenderBox, RenderObject, Widget};
use agape_core::GlobalId;
use agape_layout::{BoxSizing, IntrinsicSize};
use std::fs;
use std::path::Path;
use usvg::Tree;

#[derive(Debug)]
pub struct Svg {
    id: GlobalId,
    data: Tree,
}

impl Svg {
    /// Open an svg file.
    pub fn open<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let data = fs::read(path)?;
        let options = usvg::Options::default();
        let tree = Tree::from_data(&data, &options)?;

        Ok(Self {
            id: GlobalId::new(),
            data: tree,
        })
    }
}

impl Widget for Svg {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn build(&self) -> RenderBox {
        let size = self.data.size();
        let object = RenderObject::Svg(self.data.clone());
        let description = LayoutDescription {
            intrinsic_size: IntrinsicSize {
                width: BoxSizing::Fixed(size.width()),
                height: BoxSizing::Fixed(size.height()),
            },
            ..Default::default()
        };

        RenderBox::new(self.id, description, object)
    }
}
