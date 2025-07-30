use agape::hstack;
use agape::widgets::*;

#[test]
fn traverse_nested_children() {
    let text = Text::new("");
    let id = text.id();
    let hstack = hstack! {
        hstack!{ text }
    };

    let mut length = 0;
    hstack.traverse(&mut |child| {
        length += 1;
        if length == 2 {
            assert_eq!(child.id(), id);
        }
    })
}
