use crate::pixel::Pixel;

pub trait Draw {
    fn draw(&self, screen: &mut Vec<Vec<Option<Pixel>>>);
}
