use helium::widgets::{Rect, Widget, WidgetIterator};

#[test]
fn widget_iter_returns_self(){
    let rect = Rect::new(100.0,50.0);
    let mut iter = rect.iter();
    assert_eq!(iter.next().unwrap().id(), rect.id());
}