use agape::resources::EventQueue;
use agape::widgets::Widget;
use agape::{
    Resources, hstack,
    system::{IntoSystem, System},
};
use agape_core::{Position, Size};

#[test]
fn insert_and_get_resource() {
    let size = Size::new(100.0, 100.0);
    let mut resources = Resources::new();
    resources.insert(size);
    assert_eq!(*resources.get::<Size>().unwrap(), size);
}

#[test]
fn get_trait_from_resource() {
    let hstack = hstack! {};
    let widget: Box<dyn Widget> = Box::new(hstack);
    let mut resources = Resources::new();
    resources.insert(widget);
    resources.get_mut::<Box<dyn Widget>>().unwrap();
}

#[test]
fn function_system() {
    let event_queue = EventQueue::new();
    let mut resources = Resources::new();
    resources.insert(Position::unit(20.0));

    let func = |resources: &mut Resources| {
        resources.get::<Position>().unwrap();
    };
    let mut system = func.into_system();
    system.run(&mut resources, &event_queue);
}

#[test]
fn event_system() {
    #[derive(PartialEq, Debug)]
    struct Dummy;

    let mut event_queue = EventQueue::new();
    event_queue.push(Dummy);

    let mut resources = Resources::new();
    resources.insert(Position::unit(20.0));

    let func = |_: &mut Resources, event: &Dummy| assert_eq!(event, &Dummy);

    let mut system = func.into_system();
    system.run(&mut resources, &event_queue);
}

#[test]
fn event_system_only_runs_when_present() {
    let event_queue = EventQueue::new();
    let mut resources = Resources::new();
    resources.insert(Position::unit(20.0));

    let func = |_resources: &mut Resources, _event: &()| {
        panic!();
    };
    let mut system = func.into_system();
    system.run(&mut resources, &event_queue);
}
