use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};
use helium_core::{Color, GlobalId, IntoColor, Position, Rgba, Size};

/// A [`View`] is a primitive object that performs the rendering to the screen.
pub trait View{
    fn id(&self) -> GlobalId;
    fn set_size(&mut self, size: Size);
    fn set_position(&mut self, position: Position);
    fn render(&self,pixmap: &mut Pixmap);
}

#[derive(Default)]
pub struct RectView{
    id: GlobalId,
    position: Position,
    size: Size,
    color: Color<Rgba>,
}


impl RectView{
    pub fn new(color: impl IntoColor<Rgba>) -> Self{
        let color = color.into_color();
        
        Self{
            id: GlobalId::new(),
            position: Position::default(),
            size: Size::default(),
            color
        }
    }
}

impl View for RectView {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn set_position(&mut self, position: Position) {
        self.position = position
    }

    fn set_size(&mut self, size: Size) {
        self.size = size
    }

    fn render(&self, pixmap: &mut Pixmap) {
        pixmap.fill(tiny_skia::Color::WHITE); let mut paint = Paint::default();
        paint.set_color(tiny_skia::Color::BLACK);
        let rect = tiny_skia::Rect::from_xywh(0.0,0.0,50.0,50.0).unwrap();
        let path = PathBuilder::from_rect(rect);
        pixmap.fill_path(&path,&paint,FillRule::Winding,Transform::identity(), None);
    }
}