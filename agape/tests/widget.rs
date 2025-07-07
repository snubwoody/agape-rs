use agape::{hstack, vstack};
use agape::widgets::{Text, Widget, WidgetIterator};

#[test]
fn widget_iter_includes_self(){
    let hstack = hstack! {
        Text::new("")
    };
    
    let id = hstack.id();
    let mut iter = hstack.iter();
    let first = iter.next().unwrap();
    assert_eq!(id, first.id());
}

#[test]
fn widget_iter_has_nested_children(){
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