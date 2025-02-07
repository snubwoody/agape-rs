use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum LayoutError {
    #[error("Widget(id:{child_id}) is out of it's parent's (id:{parent_id}) bounds")]
    OutOfBounds { parent_id: String, child_id: String },
    #[error("Widget(id:{0})'s children have overflown")]
    Overflow(String),
}
