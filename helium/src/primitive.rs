// TODO impl from
pub enum Primitive {
    Text(&'static str),
    Image(image::DynamicImage),
    Icon,
    Rect,
    Circle,
}
