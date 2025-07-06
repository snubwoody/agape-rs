use agape::system::Resources;
use agape::system::{IntoSystem, System};
use agape_core::{Position, Size};

#[test]
fn insert_and_get_resource() {
    let size = Size::new(100.0, 100.0);
    let mut resources = Resources::new();
    resources.insert(size);
    assert_eq!(*resources.get::<Size>().unwrap(), size);
}

#[test]
fn function_system() {
    let mut resources = Resources::new();
    resources.insert(Position::unit(20.0));

    let func = |resources: &mut Resources| {
        resources.get::<Position>().unwrap();
    };
    let mut system = func.into_system();
    system.run(&mut resources);
}
