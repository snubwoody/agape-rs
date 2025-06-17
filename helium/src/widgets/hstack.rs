use crate::{impl_layout, impl_style, widgets::Widget, Color};
use crystal::{AxisAlignment, HorizontalLayout, Layout};
use helium_core::{colors::TRANSPARENT, GlobalId, Rgba};
use helium_renderer::{IntoSurface, RectSurface};

use super::{LayoutConfig, LayoutType, WidgetBody};

/// A [`Widget`] that places it's children horizontally. The `hstack!` macro
/// provides convienient initialization and is mostly how you be creating an `HStack`
/// most of the time.
///
/// # Example using `hstack!`
/// ```
/// use helium::{hstack,widgets::{Circle,Text}};
///
/// hstack!{
/// 	Circle::new(15),
/// 	Text::new("Hello world")
/// };
///
/// ```
///
/// You can also simply use the struct method for initialization if you choose to,
/// which is what `hstack!` expands to
///
/// ```
/// use helium::widgets::{Text,HStack};
///
/// HStack::new()
/// 	.add_child(Text::new("Hello"))
/// 	.add_child(Text::new("World"));
///
/// ```
///
pub struct HStack {
    id: GlobalId,
    children: Vec<Box<dyn Widget>>,
    color: Color<Rgba>,
    corner_radius: u32,
    layout: HorizontalLayout,
}

// TODO add get methods
impl HStack {
    pub fn new() -> Self {
        HStack {
            id: GlobalId::default(),
            color: TRANSPARENT,
            children: vec![],
            corner_radius: 0,
            layout: HorizontalLayout::new(),
        }
    }

    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

    pub fn get(&self, index: usize) -> Option<&dyn Widget> {
        self.children.get(index).map(|w| &**w)
    }

    pub fn add_child(mut self, widget: impl Widget + 'static) -> Self {
        self.children.push(Box::new(widget));
        self
    }

    pub fn padding(mut self, padding: u32) -> Self {
        self.layout.padding = padding;
        self
    }

	/// Set the spacing between children
	/// 
	/// # Example
	/// 
	/// ```
	/// use helium::{hstack,widgets::Text};
	/// 
	/// hstack!{
	/// 	Text::new("Hello"),
	/// 	Text::new("world!"),
	/// }
	/// .spacing(24);
	/// ```
    pub fn spacing(mut self, spacing: u32) -> Self {
        self.layout.spacing = spacing;
        self
    }

    pub fn align_center(mut self) -> Self {
        self.layout.main_axis_alignment = AxisAlignment::Center;
        self.layout.cross_axis_alignment = AxisAlignment::Center;
        self
    }

    pub fn main_axis_alignment(mut self, alignment: AxisAlignment) -> Self {
        self.layout.main_axis_alignment = alignment;
        self
    }

    pub fn cross_axis_alignment(mut self, alignment: AxisAlignment) -> Self {
        self.layout.cross_axis_alignment = alignment;
        self
    }

    impl_layout!();
    impl_style!();
}

// TODO test this
impl Widget for HStack {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn layout(&self, renderer: &mut helium_renderer::Renderer) -> Box<dyn Layout> {
        let children_layout: Vec<Box<dyn Layout>> = self
            .children
            .iter()
            .map(|widget| widget.layout(renderer))
            .collect();

        let HorizontalLayout {
            spacing,
            padding,
            intrinsic_size,
            main_axis_alignment,
            cross_axis_alignment,
            constraints,
            ..
        } = self.layout;

        // TODO use builder pattern?
        let layout = HorizontalLayout {
            id: self.id.clone(),
            spacing,
            padding,
            intrinsic_size,
            cross_axis_alignment,
            main_axis_alignment,
            constraints,
            children: children_layout,
            ..Default::default()
        };

        Box::new(layout)
    }

	fn build(&self,renderer: &mut helium_renderer::Renderer) -> WidgetBody {
		let children: Vec<Box<WidgetBody>> = self
            .children
            .iter()
            .map(|widget| Box::new(widget.build(renderer)))
            .collect();

        let HorizontalLayout {
            spacing,
            padding,
            intrinsic_size,
            main_axis_alignment,
            cross_axis_alignment,
            ..
        } = self.layout;

		let layout = LayoutConfig::new()
			.spacing(spacing)
			.padding(padding)
			.intrinsic_size(intrinsic_size)
			.main_axis_alignment(main_axis_alignment)
			.cross_axis_alignment(cross_axis_alignment)
			.layout(LayoutType::HorizontalLayout);

		
		// FIXME
		let primitive = RectSurface::new(0.0, 0.0)
			.color(self.color.clone())
			.corner_radius(self.corner_radius as f32)
			.into_surface();

        WidgetBody{
			id: self.id,
			primitive,
			layout,
			children
		}
	}

    fn draw(&self, layout: &dyn crystal::Layout, renderer: &mut helium_renderer::Renderer) {
		let primitive = RectSurface::from(layout)
			.color(self.color.clone())
			.corner_radius(self.corner_radius as f32);

        renderer.draw([primitive]);
    }

    fn children(&self) -> Vec<&dyn Widget> {
        self.children
            .iter()
            .map(|child| child.as_ref())
            .collect::<Vec<_>>()
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        self.children.as_mut_slice()
    }
}

/// Creates an [`HStack`].  
/// 
/// `hstack!` allows [`HStack`]'s to be declared in a more declarative manner.
/// ```
/// use helium::{hstack,widgets::{Text}};
///
/// hstack!{
/// 	Text::new("Hello"),
/// 	Text::new("world"),
/// };
///
/// ```
// TODO add vec-like syntax hstack[widget;10]
#[macro_export]
macro_rules! hstack {
	($($child:expr), + $(,)?) => {
		{
			$crate::widgets::HStack::new()
			$(.add_child($child))*
		}
	};
	
	()=>{
		$crate::widgets::HStack::new()	
	};

	
}
