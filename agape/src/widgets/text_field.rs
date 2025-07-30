use crate::impl_style;
use crate::style::BoxStyle;
use crate::widgets::{RenderBox, Text, Widget};
use agape_core::GlobalId;
use winit::event::ElementState;
use winit::keyboard::{Key, NamedKey};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct TextField {
    id: GlobalId,
    pub text: Text,
    pub style: BoxStyle,
}

impl TextField {
    pub fn new() -> Self {
        Self::default()
    }

    impl_style!();
}

impl Widget for TextField {
    fn key_input(&mut self, key: &Key, state: &ElementState, text: &Option<String>) {
        // Prevent double input
        if !state.is_pressed() {
            return;
        }

        if let Key::Named(key) = key {
            if let NamedKey::Backspace = key
                && !self.text.text.is_empty()
            {
                self.text.text.pop();
            }
        }

        if let Some(text) = text {
            match text.as_ref() {
                // Skip escape characters
                "\t" | "\r" | "\u{8}" => {}
                _ => {
                    self.text.text.push_str(text);
                }
            }
        }
    }

    fn id(&self) -> GlobalId {
        self.id
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![&self.text]
    }

    fn traverse(&self, f: &mut dyn FnMut(&dyn Widget)) {
        f(&self.text);
        self.text.traverse(f);
    }

    fn traverse_mut(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        f(&mut self.text);
        self.text.traverse_mut(f);
    }

    fn build(&self) -> RenderBox {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::widgets::{TextField, Widget};
    use winit::event::ElementState;
    use winit::keyboard::{Key, NamedKey};

    #[test]
    fn text_input() {
        let key = Key::Character("A".into());
        let state = ElementState::Pressed;
        let text = Some("A".to_string());

        let mut text_field = TextField::new();
        text_field.key_input(&key, &state, &text);

        assert_eq!(text_field.text.text, "A");
    }

    #[test]
    fn ignore_key_when_released() {
        let key = Key::Character("B".into());
        let state = ElementState::Released;
        let text = Some("B".to_string());

        let mut text_field = TextField::new();
        text_field.key_input(&key, &state, &text);

        assert_eq!(text_field.text.text, "");
    }

    #[test]
    fn erase_text() {
        let text = "Pizza";
        let mut text_field = TextField::new();

        for char in text.chars() {
            let key = Key::Character(char.to_string().into());
            let state = ElementState::Pressed;
            let text = Some(char.to_string());
            text_field.key_input(&key, &state, &text);
        }

        assert_eq!(text_field.text.text, "Pizza");

        let key = Key::Named(NamedKey::Backspace);
        let state = ElementState::Pressed;

        text_field.key_input(&key, &state, &None);
        assert_eq!(text_field.text.text, "Pizz");
    }
}
