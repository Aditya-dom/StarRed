use std::fs::File;
use std::io::BufWriter;
use std::iter::once;
use png::{Encoder, HasParameters};
use crate::vector::Vector;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Image {
    pub width: usize,
    pub height: usize,

    pixels: Vec<Vec<Vector>>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = Self::pixels(width, height);

        Self { width, height, pixels }
    }

    pub fn get(&self, x: usize, y: usize) -> Vector {
        self.pixels[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, color: Vector) {
        self.pixels[y][x] = color;
    }

    pub fn write(&self, filename: &str) {
        let file = File::create(filename).expect("failed to create image file");
        let buffer = BufWriter::new(file);

        let mut encoder = Encoder::new(buffer, self.width as u32, self.height as u32);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);

        let mut writer = encoder.write_header().expect("failed to write png header");
        writer.write_image_data(&self.bytes()).expect("failed to write png data");
    }

    fn pixels(width: usize, height: usize) -> Vec<Vec<Vector>> {
        (0..height).map(|_| {
            (0..width).map(|_| {

                Vector::default()

            }).collect()
        }).collect()
    }

    fn bytes(&self) -> Vec<u8> {
        self.pixels.iter().flat_map(|row| {
            row.iter().flat_map(|color| {
                once(Self::byte(color.x))
                    .chain(once(Self::byte(color.y)))
                    .chain(once(Self::byte(color.z)))
            })
        }).collect()
    }

    fn byte(mut channel: f64) -> u8 {
        if channel > 1.0 { channel = 1.0 }
        if channel < 0.0 { channel = 0.0 }

        (channel * 255.0).round() as u8
    }
}

#[cfg(test)]
mod test;