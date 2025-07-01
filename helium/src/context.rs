use crate::AppEvent;
use crate::widgets::{Widget, WidgetState};
use crystal::Layout;
use helium_core::{GlobalId, Position};
use std::collections::HashMap;
use winit::event::{ElementState, MouseButton, WindowEvent};

/// Global app context which keeps track of important
/// information such as the current mouse position and
/// the state of each widget.
#[derive(Debug)]
pub struct Context {
    mouse_position: Position,
    /// Keeps track of the state of all widgets in the
    /// widget tree.
    state: HashMap<GlobalId, WidgetState>,
    layout: Box<dyn Layout>,
    events: Vec<AppEvent>,
}

impl Context {
    /// Create a new context object
    pub fn new(widget: &impl Widget) -> Self {
        let mut state = HashMap::new();
        for w in widget.iter() {
            state.insert(w.id(), WidgetState::Resting);
        }
        let layout = widget.layout();

        Self {
            mouse_position: Position::default(),
            layout,
            state,
            events: Vec::new(),
        }
    }
    
    pub(crate) fn update_mouse_pos(&mut self, mouse_position: Position) {
        self.mouse_position = mouse_position;
    }

    pub(crate) fn set_layout(&mut self, layout: Box<dyn Layout>) {
        self.layout = layout;
    }

    pub fn layout(&self) -> &dyn Layout {
        &*self.layout
    }

    pub fn mouse_pos(&self) -> Position {
        self.mouse_position
    }

    pub fn state(&self) -> &HashMap<GlobalId, WidgetState> {
        &self.state
    }

    /// Get the state of a [`Widget`].
    pub fn get_state(&self, id: &GlobalId) -> Option<&WidgetState> {
        self.state.get(id)
    }

    /// Set the state of a [`Widget`].
    pub fn set_state(&mut self, id: GlobalId, state: WidgetState) {
        self.state.insert(id, state);
    }

    pub(crate) fn handle_event(&mut self, event: WindowEvent) {
        match event { 
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = position.into();
            },
            WindowEvent::MouseInput {state, button, .. } => self.handle_click(state, button),
            _ => {}
        }
    }
    
    fn handle_click(&mut self,state: ElementState,button: MouseButton) {
        let mut clicked_ids = vec![];
        
        for layout in self.layout.iter(){
            let widget_state = self.get_state(&layout.id()).unwrap();
            
            if let ElementState::Pressed = state
                && MouseButton::Left == button
                && WidgetState::Hovered == *widget_state 
            {
                clicked_ids.push(layout.id());
            }
        }
        
        clicked_ids.iter().for_each(|&id| {
            self.events.push(AppEvent::Clicked(id));
            self.set_state(id, WidgetState::Clicked)
        });
    }
    
    /// Go over every widget and update its state based on current
    /// conditions like the mouse position. This is called every frame.
    pub fn update_state(&mut self) {
        let position = self.mouse_position;

        let hovered_ids: Vec<GlobalId> = self
            .layout
            .iter()
            .filter(|layout| layout.bounds().within(&position))
            .map(|layout| layout.id())
            .collect();

        // Get the rest of the widgets that aren't hovered
        let resting_ids: Vec<GlobalId> = self
            .layout
            .iter()
            .filter(|layout| !hovered_ids.contains(&layout.id()))
            .map(|layout| layout.id())
            .collect();

        for id in resting_ids {
            self.set_state(id, WidgetState::Resting);
        }

        for id in hovered_ids {
            if self.get_state(&id).unwrap() == &WidgetState::Resting {
                self.events.push(AppEvent::Hovered(id));
                self.set_state(id, WidgetState::Hovered);
            }
        }
    }

    /// Get a slice of the current app events.
    pub fn query_events(&self) -> &[AppEvent] {
        self.events.as_slice()
    }

    /// Clear all the events.
    pub(crate) fn clear_events(&mut self) {
        self.events.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::widgets::Rect;
    use crate::{hstack, vstack};
    use crystal::LayoutSolver;
    use helium_core::Size;
    
    #[test]
    fn init_context() {
        let widget = hstack! {
            Rect::new(100.0,100.0),
            Rect::new(100.0,100.0),
        };

        let root_id = widget.id();
        let id1 = widget.children()[0].id();
        let id2 = widget.children()[1].id();

        let ctx = Context::new(&widget);

        assert!(ctx.state.get(&root_id).is_some());
        assert!(ctx.state.get(&id1).is_some());
        assert!(ctx.state.get(&id2).is_some());
    }

    #[test]
    fn state_has_nested_children() {
        let widget = hstack! {
            Rect::new(100.0,100.0),
            vstack! {
                Rect::new(100.0,100.0),
            }
        };

        let vstack = widget.children()[1];
        let rect_id = vstack.children()[0].id();

        let ctx = Context::new(&widget);
        assert!(ctx.state.get(&rect_id).is_some());
    }

    #[test]
    fn hover_widget() {
        let widget = Rect::new(100.0, 100.0);
        let id = widget.id();
        let mut layout = widget.layout();

        LayoutSolver::solve(&mut *layout, Size::unit(300.0));

        let mut ctx = Context::new(&widget);
        ctx.set_layout(layout);
        ctx.update_mouse_pos(Position::unit(50.0));
        ctx.update_state();
        assert_eq!(ctx.get_state(&id).unwrap(), &WidgetState::Hovered);
    }
    
    #[test]
    fn click_widget() {
        let widget = Rect::new(100.0, 100.0);
        let id = widget.id();

        let mut cx = Context::new(&widget);
        cx.set_state(id, WidgetState::Hovered);
        cx.handle_click(ElementState::Pressed, MouseButton::Left);
        let state = cx.get_state(&id).unwrap();
        
        assert_eq!(state, &WidgetState::Clicked);
        assert!(cx.query_events().contains(&AppEvent::Clicked(id)));
    }
    
    #[test]
    fn dont_click_widget_if_not_hovered(){
        
    }

    #[test]
    fn default_resting_state(){
        let widget = hstack! {};
        let cx = Context::new(&widget);
        let state = cx.get_state(&widget.id()).unwrap();
        assert_eq!(state,&WidgetState::Resting);
    }
}
