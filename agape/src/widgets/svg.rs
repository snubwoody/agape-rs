use crate::widgets::{LayoutDescription, RenderBox, RenderObject, Widget};
use agape_core::GlobalId;
use agape_layout::{BoxSizing, IntrinsicSize};
use std::fs;
use std::path::Path;
use std::rc::Rc;
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
///     let svg = Svg::from_data(data.as_bytes())?;
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
    data: Rc<Tree>,
}

impl Svg {
    /// Open and parse an svg file.
    pub fn open<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let data = fs::read(path)?;
        let options = usvg::Options::default();
        let tree = Tree::from_data(&data, &options)?;

        Ok(Self {
            id: GlobalId::new(),
            data: Rc::new(tree),
        })
    }

    /// Parse SVG data.
    pub fn from_data(data: &[u8]) -> crate::Result<Self> {
        let options = usvg::Options::default();
        let tree = Tree::from_data(data, &options)?;
        Ok(Self {
            id: GlobalId::new(),
            data: Rc::new(tree),
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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
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
        let svg = Svg::from_data(data.as_bytes()).unwrap();
        let width = svg.data.size().width();
        let height = svg.data.size().height();

        assert_eq!(width, 50.0);
        assert_eq!(height, 200.0);
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

        let id: u8 = rand::random();
        let path = format!("temp/svg-{id}.svg");
        let mut file = File::create(&path)?;
        file.write_all(data.as_bytes())?;

        let svg = Svg::open(&path)?;
        let width = svg.data.size().width();
        let height = svg.data.size().height();

        assert_eq!(width, 50.0);
        assert_eq!(height, 200.0);
        fs::remove_file(&path)?;

        Ok(())
    }
}
