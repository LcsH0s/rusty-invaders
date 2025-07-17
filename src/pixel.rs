#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum PixelColor {
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

    pub fn argb(&self) -> (u8, u8, u8, u8) {
        match self {
            PixelColor::Black => (0xff, 0, 0, 0),
            PixelColor::White => (0xff, 0xff, 0xff, 0xff),
            PixelColor::Red => (0xff, 0xff, 0, 0),
            PixelColor::Green => (0xff, 0, 0xff, 0),
            PixelColor::Blue => (0xff, 0, 0, 0xff),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Pixel {
    color: PixelColor,
}

impl Pixel {
    pub fn new(color: PixelColor) -> Self {
        Self { color }
    }

    pub fn black() -> Self {
        Self {
            color: PixelColor::Black,
        }
    }

    pub fn white() -> Self {
        Self {
            color: PixelColor::White,
        }
    }
    pub fn red() -> Self {
        Self {
            color: PixelColor::Red,
        }
    }
    pub fn green() -> Self {
        Self {
            color: PixelColor::Green,
        }
    }
    pub fn blue() -> Self {
        Self {
            color: PixelColor::Blue,
        }
    }

    pub fn color(&self) -> PixelColor {
        self.color
    }
}

pub fn pixels_from_schema(schema: &[&[u8]], pixel: Pixel) -> Vec<Vec<Option<Pixel>>> {
    let mut pixels = Vec::with_capacity(schema.len());

    for line in schema {
        let mut pixel_line = Vec::with_capacity(line.len());
        for p in *line {
            match *p {
                1 => pixel_line.push(Some(pixel)),
                _ => pixel_line.push(None),
            }
        }
        pixels.push(pixel_line);
    }

    pixels
}
