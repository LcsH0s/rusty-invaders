use crate::pixel::Pixel;

pub trait Draw {
    fn draw(&self, screen: &mut Vec<Vec<Option<Pixel>>>);
}

pub trait Kinetic {
    fn translate(&mut self) -> bool;
}
