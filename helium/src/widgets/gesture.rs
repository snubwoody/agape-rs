use helium_core::{GlobalId, Position};

pub struct GestureDetector {
    id: GlobalId,
    mouse_pos: Position,
    hovered: bool,
}
