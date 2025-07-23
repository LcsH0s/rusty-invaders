use crate::{
    game::WINDOW_PIXEL_HEIGHT,
    pixel::Pixel,
    traits::{Draw, Kinetic},
};
pub const SHOT_DIMS: (usize, usize) = (1, 2);

pub enum ShotType {
    Player,
    Enemy,
}

pub struct Shot {
    pos: (usize, usize),
    stype: ShotType,
}

impl Shot {
    pub fn new(pos: (usize, usize), stype: ShotType) -> Self {
        Self { pos, stype }
    }
}

impl Draw for Shot {
    fn draw(&self, screen: &mut Vec<Vec<Option<crate::pixel::Pixel>>>) {
        match self.stype {
            ShotType::Player => {
                if self.pos.1 == 0 {
                    screen[0][self.pos.0] = Some(Pixel::white());
                } else {
                    screen[self.pos.1][self.pos.0] = Some(Pixel::white());
                    screen[self.pos.1 - 1][self.pos.0] = Some(Pixel::white());
                }
            }
            ShotType::Enemy => {
                if self.pos.1 == WINDOW_PIXEL_HEIGHT {
                    screen[WINDOW_PIXEL_HEIGHT][self.pos.0] = Some(Pixel::white());
                } else {
                    screen[self.pos.1][self.pos.0] = Some(Pixel::white());
                    screen[self.pos.1 - 1][self.pos.0] = Some(Pixel::white());
                }
            }
        }
    }
}

impl Kinetic for Shot {
    fn translate(&mut self) -> bool {
        match self.stype {
            ShotType::Player => {
                if self.pos.1 == 0 {
                    false
                } else {
                    self.pos.1 = self.pos.1 - 1;
                    true
                }
            }
            ShotType::Enemy => {
                if self.pos.1 == WINDOW_PIXEL_HEIGHT - 1 {
                    false
                } else {
                    self.pos.1 = self.pos.1 + 1;
                    true
                }
            }
        }
    }
}
