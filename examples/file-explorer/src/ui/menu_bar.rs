use crate::FileInfo;
use agape::state::StateCell;
use agape::widgets::*;
use agape::{Color, hstack};
use std::fs;

#[derive(Debug, Clone, Default)]
pub struct MenuBar;

impl StatelessWidget for MenuBar {
    type Widget = HStack;

    fn build(&self) -> Self::Widget {
        hstack![
            Button::new(Icon::asset("icons/regular/caret-left.svg").fixed(16.0, 16.0))
                .padding(12)
                .corner_radius(12)
                .background_color(230),
            Button::new(Icon::asset("icons/regular/caret-right.svg").fixed(16.0, 16.0))
                .padding(12)
                .corner_radius(12)
                .background_color(230),
        ]
        .spacing(12)
        .padding(12)
        .fill_width()
        .background_color(250)
    }
}
