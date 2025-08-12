//! [`Widget`]'s describe what you want to present onto the screen. Agape tries to provide
//! as many [`Widget`]'s as possible for various uses such as [`Text`],[`Button`],[`HStack`]
//! and [`VStack`], and the list goes on. Every widget must implement the [`Widget`] trait.
mod container;
mod hstack;
pub mod image;
mod rect;
mod svg;
mod text;
mod vstack;

use agape_core::GlobalId;
use agape_layout::Layout;
use agape_renderer::Renderer;
pub use container::Container;
pub use hstack::*;
pub use image::Image;
pub use rect::*;
pub use svg::Svg;
pub use text::Text;
use tiny_skia::Pixmap;
pub use vstack::*;

pub trait View {
    fn update(&mut self) {}

    fn view(&self) -> Box<dyn Widget>;
}

pub trait Widget {
    /// Get the `id` of the [`Widget`]
    fn id(&self) -> GlobalId;

    fn layout(&self, _: &mut Renderer) -> Box<dyn Layout>;
    /// Draw the widget to the screen.
    fn render(&self, _: &mut Pixmap, _: &mut Renderer, _: &dyn Layout) {}
}
