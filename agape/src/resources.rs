//! A resource is anything that needs to be accessed globally by different
//! systems, common items such as the cursor position or the window size.
//!
//! Resources work on types, `T`, so avoid setting primitive or commonly used types
//! like `String` or `Box` as it will make tracking things much harder.
use agape_core::{Position, Size};
use agape_layout::Layout;
use bevy_ecs::prelude::Resource;
use winit::event::WindowEvent;

/// The current cursor position.
#[derive(Debug, Default, Copy, Clone, Resource)]
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

    pub fn current(&self) -> Position {
        self.current
    }

    pub fn previous(&self) -> Position {
        self.previous
    }

    /// Check if the widget was just hovered.
    pub fn just_hovered(&self, layout: &dyn Layout) -> bool {
        let bounds = layout.bounds();
        !bounds.within(&self.previous) && bounds.within(&self.current)
    }
}

/// The window size.
#[derive(Debug, Default, Copy, Clone)]
pub struct WindowSize(pub Size);

#[derive(Debug, Default, Resource)]
pub struct EventQueue {
    events: Vec<WindowEvent>,
    frame_count: u32,
}

impl EventQueue {
    pub fn new() -> Self {
        Self::default()
    }

    /// Push an event to the queue.
    pub fn push(&mut self, event: WindowEvent) {
        self.events.push(event);
    }

    pub fn events(&self) -> &[WindowEvent] {
        self.events.as_slice()
    }

    pub fn tick(&mut self) {
        self.frame_count += 1;
    }

    pub fn clear(&mut self) {
        if self.frame_count > 3 {
            self.events.clear();
            self.frame_count = 0;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
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
