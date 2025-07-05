use agape::{Context, hstack, system::{IntoSystem, System}, AppEvent};
use agape_core::GlobalId;

#[test]
fn function_system() {
    let widget = hstack! {};
    let mut cx = Context::new(&widget);
    let func = |cx: &mut Context| println!("I am a function system");
    let mut system = func.into_system();
    system.run(&mut cx,AppEvent::Hovered(GlobalId::new()));
}
