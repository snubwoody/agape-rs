//! A [`Widget`] describe what you want to present onto the screen. `agape` tries to
//! provide as many [`Widget`]'s as possible for various uses such as [`Text`],[`Button`],
//! [`HStack`] and [`VStack`], and so on. There are two ways of creating widgets.
//!
//! ## Wrap another widget
//! For most simple use cases, you may wrap another widget in a function.
//!
//! ```
//! use agape::{widgets::*,Color};
//!
//! fn CtaButton() -> impl Widget{
//!     Button::text("Get started")
//!         .background_color(Color::rgb(100,25,255))
//!         .padding(16)
//!         .corner_radius(12)
//! }
//! ```
//!
//! Widgets use the [builder pattern](https://refactoring.guru/design-patterns/builder) to
//! configure properties, most widgets will have extra properties that can be configured,
//! check their docs.
//!
//! ## `#[derive(Widget)]`
//! For widgets that need to react and manage state the [`Widget`] derive macro is provided,
//! the widget will need a [`GlobalId`] and a child widget.
//!
//! ```
//! use agape::{Widget, widgets::{HStack,Button}, GlobalId, hstack};
//!
//! #[derive(Widget)]
//! struct Menu{
//!     id: GlobalId,
//!     #[child]
//!     child: HStack
//! }
//!
//! impl Menu{
//!     pub fn new() -> Self{
//!         let child = hstack![
//!             Button::text("File"),
//!             Button::text("Edit"),
//!             Button::text("View"),
//!             Button::text("About"),
//!         ]
//!             .spacing(12)
//!             .padding(16);
//!
//!         Self{
//!             id: GlobalId::new(),
//!             child
//!         }
//!     }
//! }
//! ```
//!
//! State is handled using messages, a [`Message`] is any rust struct that implements
//! `Any + Debug`, by using messages coupling is reduces and widgets are able to be
//! more independent. To make a widget stateful, i.e. to react to messages, use the
//! `#[interactive]` attribute.
//!
//! ```
//! use agape::{Widget, GlobalId, widgets::{Button,Text}, hstack, MessageQueue};
//! use agape::widgets::HStack;
//!
//!
//! #[derive(Debug,Copy, Clone)]
//! struct Marco;
//! #[derive(Debug,Copy, Clone)]
//! struct Polo;
//!
//! #[derive(Widget)]
//! #[interactive]
//! struct Menu{
//!     id: GlobalId,
//!     #[child]
//!     child: HStack
//! }
//!
//! impl Menu{
//!     pub fn new() -> Self{
//!         let child = hstack![
//!             Button::text("Marco")
//!                 .on_click(|messages|messages.add(Marco)),
//!             Button::text("Polo")
//!                 .on_click(|messages|messages.add(Polo))
//!         ];
//!
//!         Self{
//!             id: GlobalId::new(),
//!             child
//!         }     
//!     }   
//!
//!     pub fn update(&mut self, messages: &mut MessageQueue){
//!         if messages.has::<Marco>(){
//!             println!("Marco!")
//!         }   
//!     
//!         if messages.has::<Polo>(){
//!             println!("Polo!")
//!         }      
//!     }
//! }
//! ```
mod button;
mod container;
mod hstack;
mod icon;
pub mod image;
mod rect;
mod svg;
mod text;
mod text_field;
mod vstack;

use crate::assets::AssetManager;
use crate::message::MessageQueue;
use crate::state::Context;
use agape_core::GlobalId;
use agape_layout::Layout;
use agape_renderer::Renderer;
pub use button::*;
pub use container::Container;
pub use hstack::*;
pub use icon::Icon;
pub use image::Image;
pub use rect::*;
pub use svg::Svg;
pub use text::Text;
pub use text_field::TextField;
pub use vstack::*;

/// A `Widget` is anything that can ultimately be drawn to the screen. Widgets internally
/// can be composed of anything, but each widget must have a [`GlobalId`] and a [`Layout`].
/// See the [module docs] for more info.
///
/// [module docs]: crate::widgets
pub trait Widget {
    /// Get the `id` of the [`Widget`].
    fn id(&self) -> GlobalId;

    /// Construct a [`Layout`] to solve layout for the whole
    /// widget tree.
    fn layout(&self, _: &mut Renderer) -> Box<dyn Layout>;

    /// Draw the widget to the screen.
    fn render(&self, _: &mut Renderer, _: &dyn Layout);

    fn children(&self) -> Vec<&dyn Widget>;

    /// Traverse the widgets children. Note that this doesn't
    /// include the widget itself.
    fn traverse(&mut self, _: &mut dyn FnMut(&mut dyn Widget));

    fn get_assets(&mut self, _: &AssetManager) {}

    /// Called every frame.
    fn tick(&mut self, _: &mut MessageQueue) {}
    fn click(&mut self, _: &mut MessageQueue) {}
    fn hover(&mut self, _: &mut MessageQueue) {}

    fn mouse_entered(&mut self, _: &mut MessageQueue) {}
    fn mouse_left(&mut self, _: &mut MessageQueue) {}
}

pub trait StatelessWidget {
    type Widget: Widget;

    fn build(&self, ctx: &mut Context) -> Self::Widget;
}

/// An iterator over a tree of widgets.
pub struct WidgetIter<'a> {
    stack: Vec<&'a dyn Widget>,
}

impl<'a> Iterator for WidgetIter<'a> {
    type Item = &'a dyn Widget;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(widget) = self.stack.pop() {
            self.stack.extend(widget.children());
            return Some(widget);
        }
        None
    }
}

impl dyn Widget {
    pub fn iter(&self) -> WidgetIter<'_> {
        WidgetIter { stack: vec![self] }
    }
}
