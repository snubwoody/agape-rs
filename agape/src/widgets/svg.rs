use crate::impl_style;
use crate::style::BoxStyle;
use crate::widgets::Widget;
use agape_core::GlobalId;
use agape_layout::{EmptyLayout, IntrinsicSize, Layout};
use agape_renderer::Renderer;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use usvg::Tree;

/// Displays an SVG to the screen.
///
/// - Open an SVG file
/// ```no_run
/// use agape::widgets::Svg;
///
/// fn main() -> agape::Result<()>{
///     let svg = Svg::open("icons/menu.svg")?;
///     Ok(())
/// }
/// ```
/// - Load an SVG from memory
/// ```
/// use agape::widgets::Svg;
///
/// fn main() -> agape::Result<()>{
///     let data = "
///         <svg
///             version='1.1'
///             width='500'
///             height='500'
///             xlms='http://www.w3.org/2000/svg'
///         >
///         </svg>
///     ";
///     let svg = Svg::bytes(data.as_bytes())?;
///
///     Ok(())
/// }
/// ```
///
/// Only basic SVG functionality is supported, i.e. rendering vector graphics
/// to the screen. None of the other complex features such as animation, scripting
/// or audio is covered.
#[derive(Debug)]
pub struct Svg {
    id: GlobalId,
    data: Arc<Tree>,
    style: BoxStyle,
}

impl Svg {
    /// Open and parse an svg file.
    pub fn open<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let data = fs::read(path)?;
        Self::bytes(&data)
    }

    /// Load an SVG from in memory bytes.
    pub fn bytes(data: &[u8]) -> crate::Result<Self> {
        let options = usvg::Options::default();
        let tree = Tree::from_data(data, &options)?;
        let size = tree.size();
        let intrinsic_size = IntrinsicSize::fixed(size.width(), size.height());
        let style = BoxStyle {
            intrinsic_size,
            ..Default::default()
        };

        Ok(Self {
            id: GlobalId::new(),
            data: Arc::new(tree),
            style,
        })
    }

    impl_style! {}
}

impl Widget for Svg {
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
        let mut svg = agape_renderer::Svg::new(self.data.clone());
        svg.size = size;
        svg.position = position;
        renderer.draw_svg(svg);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::io::Write;

    #[test]
    fn parse_svg_from_data() {
        let data = "
            <svg 
                version='1.1' 
                width='50' 
                height='200' 
                xmlns='http://www.w3.org/2000/svg'
            >
            </svg>
        ";
        let svg = Svg::bytes(data.as_bytes()).unwrap();
        let width = svg.data.size().width();
        let height = svg.data.size().height();

        assert_eq!(width, 50.0);
        assert_eq!(height, 200.0);
    }

    #[test]
    fn inferred_size() {
        let data = "
            <svg 
                version='1.1' 
                width='50' 
                height='200' 
                xmlns='http://www.w3.org/2000/svg'
            >
            </svg>
        ";
        let svg = Svg::bytes(data.as_bytes()).unwrap();
        assert_eq!(svg.style.intrinsic_size, IntrinsicSize::fixed(50.0, 200.0));
    }

    #[test]
    fn parse_svg_from_file() -> crate::Result<()> {
        let _ = fs::create_dir("temp");
        let data = "
            <svg 
                version='1.1' 
                width='50' 
                height='200' 
                xmlns='http://www.w3.org/2000/svg'
            >
            </svg>
        ";

        let mut f = tempfile::NamedTempFile::new()?;
        let file = f.as_file_mut();
        file.write_all(data.as_bytes())?;
        let path = f.path();

        let svg = Svg::open(path)?;
        let width = svg.data.size().width();
        let height = svg.data.size().height();

        assert_eq!(width, 50.0);
        assert_eq!(height, 200.0);
        fs::remove_file(path)?;

        Ok(())
    }
}
