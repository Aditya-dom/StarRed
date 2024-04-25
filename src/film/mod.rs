use crate::image::Image;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Film {
    pub width: f64,
    pub height: f64,
    pub image: Image,
}

impl Film {
    pub fn new(width: f64, height: f64, image: Image) -> Self {
        Self { width, height, image }
    }

    pub fn pixel_ratios(&self) -> Vec<(usize, usize, f64, f64)> {
        (0..self.image.width).flat_map(|x| {
            (0..self.image.height).map(move |y| {
                let (x_ratio, y_ratio) = self.pixel_ratio(x, y);

                (x, y, x_ratio, y_ratio)
            })
        }).collect()
    }

    fn pixel_ratio(&self, x: usize, y: usize) -> (f64, f64) {
        let x = x as f64 + 0.5;
        let y = y as f64 + 0.5;

        let width = self.image.width as f64;
        let height = self.image.height as f64;

        (x / width, y / height)
    }
}

#[cfg(test)]
mod test;