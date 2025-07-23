use crate::player::Player;
use crate::traits::{Draw, Kinetic};
use crate::{pixel::Pixel, shot::Shot};

use minifb::{Key, Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};
use std::{thread, time};

pub const WINDOW_PIXEL_WIDTH: usize = 150;
pub const WINDOW_PIXEL_HEIGHT: usize = 100;

const PIXEL_SIZE: usize = 6;
const TICK_DURATION: u64 = 10;

pub struct Game {
    window: Window,
    screen: Vec<Vec<Option<Pixel>>>,

    player: Player,
    pshots: Vec<Shot>,
}

impl Game {
    pub fn new() -> Self {
        // minifb window creation
        let window = Window::new(
            "Rusty Invaders",
            WINDOW_PIXEL_WIDTH * PIXEL_SIZE,
            WINDOW_PIXEL_HEIGHT * PIXEL_SIZE,
            WindowOptions {
                ..WindowOptions::default()
            },
        )
        .unwrap();

        let player = Player::new();

        let mut screen = Vec::with_capacity(WINDOW_PIXEL_HEIGHT);
        for _ in 0..WINDOW_PIXEL_HEIGHT {
            screen.push(Vec::from([None; WINDOW_PIXEL_WIDTH]));
        }

        let mut game = Self {
            window,
            player,
            screen,
            pshots: Vec::new(),
        };

        game
    }

    pub fn run(&mut self) {
        loop {
            let tick_start = time::Instant::now();

            let keys = self.window.get_keys();
            for key in keys {
                match key {
                    Key::Left => self.player.left(),
                    Key::Right => self.player.right(),
                    Key::Space => {
                        if let Some(shot) = self.player.shoot() {
                            self.pshots.push(shot);
                        }
                    }
                    _ => (),
                }
            }

            self.pshots.retain_mut(|shot| shot.translate());

            self.render();

            let tick_elapsed = tick_start.elapsed().as_millis();
            if tick_elapsed < TICK_DURATION as u128 {
                thread::sleep(time::Duration::from_millis(
                    TICK_DURATION - tick_elapsed as u64,
                ));
            }
        }
    }

    fn render(&mut self) {
        self.draw_screen();

        let size = self.window.get_size();
        let mut buffer = DrawTarget::new(size.0 as i32, size.1 as i32);
        for (row, pixels) in self.screen.iter().enumerate() {
            for (col, pixel) in pixels.iter().enumerate().filter(|p| p.1.is_some()) {
                let pixel = pixel.unwrap();
                let mut pb = PathBuilder::new();
                pb.rect(
                    (col * PIXEL_SIZE) as f32,
                    (row * PIXEL_SIZE) as f32,
                    PIXEL_SIZE as f32,
                    PIXEL_SIZE as f32,
                );
                let path = pb.finish();
                let argb = pixel.color().argb();
                buffer.fill(
                    &path,
                    &Source::Solid(SolidSource::from_unpremultiplied_argb(
                        argb.0, argb.1, argb.2, argb.3,
                    )),
                    &DrawOptions::new(),
                );
            }
        }

        self.window
            .update_with_buffer(buffer.get_data(), size.0, size.1)
            .unwrap();
    }

    fn draw_screen(&mut self) {
        self.clear_screen();

        self.player.draw(&mut self.screen);

        for s in &self.pshots {
            s.draw(&mut self.screen);
        }
    }

    fn clear_screen(&mut self) {
        self.screen.iter_mut().for_each(|row| {
            row.iter_mut()
                .filter(|p| p.is_some())
                .for_each(|p| *p = None)
        });
    }
}
