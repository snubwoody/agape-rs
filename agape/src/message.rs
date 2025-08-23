use crate::resources::EventQueue;
use agape_core::Position;
use bevy_ecs::change_detection::ResMut;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Message {
    MouseMoved(Position),
    MouseButtonDown,
    MouseButtonUp,
}

pub fn handle_click(queue: ResMut<EventQueue>) {
    dbg!(queue);
}
