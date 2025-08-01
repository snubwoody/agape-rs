use crate::widgets::{RenderBox, Svg, Widget};
use agape_core::GlobalId;

#[derive(Debug)]
pub struct Icon {
    id: GlobalId,
    svg: Svg,
}

impl Icon {
    /// Create an icon from svg bytes.
    pub fn svg_bytes(data: &[u8]) -> crate::Result<Self> {
        let svg = Svg::from_data(data)?;
        Ok(Self {
            id: GlobalId::new(),
            svg,
        })
    }
}

impl Widget for Icon {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn build(&self) -> RenderBox {
        self.svg.build()
    }
}
