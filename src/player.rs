use crate::draw::Draw;
use crate::game::{WINDOW_PIXEL_HEIGHT, WINDOW_PIXEL_WIDTH};
use crate::pixel::{Pixel, pixels_from_schema};

const SHIP_HEIGHT: usize = 8;
const SHIP_WIDTH: usize = 15;
const SHIP_SCHEMA: [[u8; SHIP_WIDTH]; SHIP_HEIGHT] = [
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

const PLAYER_MIN_POS: usize = 2;
const PLAYER_MAX_POS: usize = WINDOW_PIXEL_WIDTH - SHIP_WIDTH - 2;

pub struct Player {
    pos: (usize, usize),
    dim: (usize, usize),
    pixels: Vec<Vec<Option<Pixel>>>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: (PLAYER_MIN_POS, WINDOW_PIXEL_HEIGHT - SHIP_HEIGHT - 2),
            dim: (SHIP_WIDTH, SHIP_HEIGHT),
            pixels: pixels_from_schema(
                &SHIP_SCHEMA[..]
                    .iter()
                    .map(|inner| inner as &[u8])
                    .collect::<Vec<&[u8]>>(),
                Pixel::white(),
            ),
        }
    }

    pub fn left(&mut self) {
        if self.pos.0 > PLAYER_MIN_POS {
            self.pos.0 = self.pos.0 - 1;
        }
    }

    pub fn right(&mut self) {
        if self.pos.0 < PLAYER_MAX_POS {
            self.pos.0 = self.pos.0 + 1;
        }
    }
}

impl Draw for Player {
    fn draw(&self, screen: &mut Vec<Vec<Option<Pixel>>>) {
        for (row, pixels) in self.pixels.iter().enumerate() {
            for (col, pixel) in pixels.iter().enumerate() {
                if pixel.is_some() {
                    screen[self.pos.1 + row][self.pos.0 + col] = Some(pixel.unwrap());
                }
            }
        }
    }
}
