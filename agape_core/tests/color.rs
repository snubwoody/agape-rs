use agape_core::IntoColor;

#[test]
fn tuple_into_color() {
    let color = (100, 100, 255, 100).into_color();
    assert_eq!(color.inner(), (100, 100, 255, 100));
}
