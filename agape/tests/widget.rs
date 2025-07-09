use agape::widgets::*;
use agape::{hstack, vstack};

#[test]
fn widget_iter_includes_self() {
    let hstack = hstack! {
        Text::new("")
    };

    let id = hstack.id();
    let mut iter = hstack.iter();
    let first = iter.next().unwrap();
    assert_eq!(id, first.id());
}

#[test]
fn widget_iter_has_nested_children() {
    let text = Text::new("");
    let id = text.id();

    let hstack = hstack! {
        vstack!{text}
    };

    let mut iter = hstack.iter();
    iter.next().unwrap();
    iter.next().unwrap();
    let third = iter.next().unwrap();

    assert_eq!(id, third.id());
}

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
