use agape_core::Position;
use agape_layout::Layout;

/// The current cursor position.
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct CursorPosition {
    current: Position,
    previous: Position,
}

impl CursorPosition {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, position: Position) {
        self.previous = self.current;
        self.current = position;
    }

    /// Get the current cursor position.
    pub fn current(&self) -> Position {
        self.current
    }

    /// Get the cursor position of the last frame.
    pub fn previous(&self) -> Position {
        self.previous
    }

    /// Returns `true` if the cursor just hovered the [`Widget`].
    pub fn just_hovered(&self, layout: &dyn Layout) -> bool {
        let bounds = layout.bounds();
        !bounds.within(&self.previous) && bounds.within(&self.current)
    }

    /// Returns `true` if the cursor is over the [`Widget`].
    pub fn is_hovered(&self, layout: &dyn Layout) -> bool {
        layout.bounds().within(&self.current)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use agape_core::Size;
    use agape_layout::{EmptyLayout, IntrinsicSize, solve_layout};

    #[test]
    fn just_hovered() {
        let mut layout = EmptyLayout::new();
        layout.intrinsic_size = IntrinsicSize::fixed(200.0, 50.0);
        solve_layout(&mut layout, Size::unit(1000.0));
        let mut cursor_pos = CursorPosition::new();
        cursor_pos.current = Position::unit(10.0);
        cursor_pos.previous = Position::unit(300.0);
        assert!(cursor_pos.just_hovered(&layout));
    }

    #[test]
    fn cursor_already_in_bounds() {
        let mut layout = EmptyLayout::new();
        layout.intrinsic_size = IntrinsicSize::fixed(200.0, 50.0);
        solve_layout(&mut layout, Size::unit(1000.0));
        let mut cursor_pos = CursorPosition::new();
        cursor_pos.current = Position::unit(10.0);
        cursor_pos.previous = Position::unit(40.0);
        assert!(!cursor_pos.just_hovered(&layout));
    }

    #[test]
    fn not_hovered() {
        let mut layout = EmptyLayout::new();
        layout.intrinsic_size = IntrinsicSize::fixed(200.0, 50.0);
        solve_layout(&mut layout, Size::unit(1000.0));
        let mut cursor_pos = CursorPosition::new();
        cursor_pos.current = Position::unit(350.0);
        cursor_pos.previous = Position::unit(400.0);
        assert!(!cursor_pos.just_hovered(&layout));
    }
}
