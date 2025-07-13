use agape_core::Size;
use winit::dpi::PhysicalSize;

#[test]
fn from_physical_size() {
    let size = Size::from(PhysicalSize::new(100, 100));
    assert_eq!(size, Size::unit(100.0));
}

#[test]
fn add_assign() {
    let mut size = Size::unit(100.0);
    size += 10.0;
    assert_eq!(size, Size::unit(110.0));
}

#[test]
fn sub_assign() {
    let mut size = Size::unit(100.0);
    size -= 10.0;
    assert_eq!(size, Size::unit(90.0));
}

#[test]
fn add_size() {
    let size = Size::unit(500.0) + Size::unit(500.0);
    assert_eq!(size, Size::unit(1000.0));
}

#[test]
fn sub_size() {
    let size = Size::unit(500.0) - Size::unit(500.0);
    assert_eq!(size, Size::unit(0.0));
}

#[test]
fn from_tuple() {
    let size = Size::from((1, 2));
    assert_eq!(size, Size::new(1.0, 2.0));
}
