use helium::hstack;
use helium::widgets::{Text, Widget, WidgetIterator};
use helium_core::GlobalId;

#[test]
fn widget_iter() {
    let hstack = hstack! {
        Text::new("Hello"),
        Text::new("Hello"),
    };

    let id1 = hstack.children()[0].id();
    let id2 = hstack.children()[1].id();

    let ids: Vec<GlobalId> = hstack.iter().map(|w| w.id()).collect();
    dbg!(ids);
    dbg!(hstack.id());
    dbg!(id1);
    dbg!(id2);
    todo!()
}
