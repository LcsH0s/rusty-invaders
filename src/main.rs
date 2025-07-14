use minifb::{Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

use std::thread::sleep;
use std::time::Duration;

const WINDOW_PIXEL_WIDTH: usize = 150;
const WINDOW_PIXEL_HEIGHT: usize = 100;

const PIXEL_SIZE: usize = 6;

const SHIP_HEIGHT: usize = 8;
const SHIP_WIDTH: usize = 15;
const SHIP_MATRIX: [[u8; SHIP_WIDTH]; SHIP_HEIGHT] = [
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

const ENEMY1_MATRIX: [[u8; 11]; 8] = [
    [0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1],
    [0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0],
];

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
enum PixelColor {
    Black,
    White,
    Red,
    Blue,
    Green,
}

impl PixelColor {
    pub fn all_variants() -> &'static [PixelColor] {
        &[Self::Black, Self::White, Self::Red, Self::Blue, Self::Green]
    }
}

struct Game {
    window: Window,
    screen: Vec<Vec<PixelColor>>,
}

impl Game {
    pub fn new() -> Self {
        let mut screen = Vec::with_capacity(WINDOW_PIXEL_HEIGHT);

        for _ in 0..WINDOW_PIXEL_HEIGHT {
            screen.push(Vec::from([PixelColor::Black; WINDOW_PIXEL_WIDTH]));
        }

        let window = Window::new(
            "Rusty Invaders",
            WINDOW_PIXEL_WIDTH * PIXEL_SIZE,
            WINDOW_PIXEL_HEIGHT * PIXEL_SIZE,
            WindowOptions {
                ..WindowOptions::default()
            },
        )
        .unwrap();

        Self { screen, window }
    }

    pub fn run(&mut self) {
        loop {
            self.draw_ship(2);
            self.render_screen();
            sleep(Duration::from_millis(100))
        }
    }

    fn render_screen(&mut self) {
        let size = self.window.get_size();
        let mut buffer = DrawTarget::new(size.0 as i32, size.1 as i32);

        for color in PixelColor::all_variants() {
            let mut pb = PathBuilder::new();

            for (row, pixels) in self.screen.iter().enumerate() {
                for (col, pixel) in pixels.iter().enumerate() {
                    if pixel == color {
                        pb.rect(
                            (col * PIXEL_SIZE) as f32,
                            (row * PIXEL_SIZE) as f32,
                            PIXEL_SIZE as f32,
                            PIXEL_SIZE as f32,
                        );
                    }
                }
            }
            let path = pb.finish();

            match color {
                PixelColor::Black => buffer.fill(
                    &path,
                    &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0)),
                    &DrawOptions::new(),
                ),
                PixelColor::White => buffer.fill(
                    &path,
                    &Source::Solid(SolidSource::from_unpremultiplied_argb(
                        0xff, 0xff, 0xff, 0xff,
                    )),
                    &DrawOptions::new(),
                ),
                PixelColor::Red => buffer.fill(
                    &path,
                    &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0, 0)),
                    &DrawOptions::new(),
                ),
                PixelColor::Green => buffer.fill(
                    &path,
                    &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0xff, 0)),
                    &DrawOptions::new(),
                ),
                PixelColor::Blue => buffer.fill(
                    &path,
                    &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0xff)),
                    &DrawOptions::new(),
                ),
            }
        }

        self.window
            .update_with_buffer(buffer.get_data(), size.0, size.1)
            .unwrap();
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: PixelColor) {
        self.screen[y][x] = color;
    }

    fn draw_pixel_matrix(&mut self, x: usize, y: usize, mat: &[&[u8]]) {
        for (row, pixels) in mat.iter().enumerate() {
            for (col, pixel) in pixels.iter().enumerate() {
                if *pixel == 1 {
                    self.set_pixel(x + col, y + row, PixelColor::White);
                }
            }
        }
    }

    fn draw_ship(&mut self, x: usize) {
        self.draw_pixel_matrix(
            x,
            WINDOW_PIXEL_HEIGHT - SHIP_HEIGHT - 2,
            &SHIP_MATRIX[..]
                .iter()
                .map(|inner| inner as &[u8])
                .collect::<Vec<&[u8]>>(),
        );
    }
}

fn main() {
    let mut game = Game::new();
    game.run();
}
