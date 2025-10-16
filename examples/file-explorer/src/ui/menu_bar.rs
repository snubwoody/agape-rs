use crate::ui::DirState;
use agape::hstack;
use agape::state::Context;
use agape::widgets::*;

#[derive(Debug, Clone, Default)]
pub struct MenuBar;

impl StatelessWidget for MenuBar {
    type Widget = HStack;

    fn build(&self, ctx: &mut Context) -> Self::Widget {
        let state = ctx.get::<DirState>();
        hstack![
            Button::new(Icon::asset("icons/regular/caret-left.svg").fixed(16.0, 16.0))
                .padding(12)
                .corner_radius(12)
                .background_color(230)
                .on_click(move |_| state.update(|state| state.previous_dir())),
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
