use image::{DynamicImage, GenericImageView};
use agape_core::{GlobalId, Position, Size};

pub struct ImageView {
    pub id: GlobalId,
    size: Size,
    position: Position,
    image: DynamicImage,
}

impl ImageView {
    pub fn new(image: DynamicImage) -> Self {
        let (width,height) = image.dimensions();
        let size = Size::from((width, height));
        
        Self{
            id: GlobalId::default(),
            size,
            position: Position::default(),
            image,
        }
    }
}

#[cfg(test)]
mod test{
    #[test]
    fn inherit_size_from_image(){
        todo!("Check if the view has the same size as the image after creating a new image")
    }
}
