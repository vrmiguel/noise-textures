use std::usize;

use crate::rand::Rand;

pub struct Noise<const HEIGHT: usize, const WIDTH: usize> {
    rand: Rand,
    buf: [[u8; WIDTH]; HEIGHT],
}

impl<const H: usize, const W: usize> Noise<H, W> {
    /// Create a new Noise struct with its
    /// internal buffer already randomized
    pub fn new() -> Self {
        let rand = Rand::new();
        let mut this = Noise {
            rand,
            buf: [[0; W]; H],
        };
        this.randomize();
        this
    }

    pub fn randomize(&mut self) {
        for i in 0..H {
            for j in 0..W {
                self.buf[i][j] = self.rand.rand_u8();
            }
        }
    }

    pub fn smooth_noise(&self, x: i32, y: i32, zoom_factor: f64) -> u8 {
        let width = W as i32;
        let height = H as i32;

        // Zooming in
        let (x, y) = (x as f64 / zoom_factor, y as f64 / zoom_factor);
        
        // Fractional parts of both
        let frac_x = x.fract();
        let frac_y = y.fract();

        // Wrap around
        let x1 = (x as i32 + width) % width;
        let y1 = (y as i32 + height) % height;

        // Neighbour values
        let x2 = ((x1 + width - 1) % width) as usize;
        let y2 = ((y1 + height - 1) % height) as usize; 

        // Smoothing the noise through bilinear interpolation
        let mut noise = 0.0;
        noise += frac_x * frac_y * (self.buf[y1 as usize][x1 as usize] as f64);
        noise += (1.0 - frac_x) * frac_y * (self.buf[y1 as usize][x2 as usize] as f64);
        noise += frac_x * (1.0 - frac_y) * (self.buf[y2 as usize][x1 as usize] as f64);
        noise += (1.0 - frac_x) * (1.0 - frac_y) * (self.buf[y2 as usize][x2 as usize] as f64);

        // dbg!(noise) as u8
        // Converting this value into 0..255

        // dbg!((255.0 * noise) as u8)
        noise as u8
    }

    pub fn turbulence(&self, x: i32, y: i32, mut zoom_factor: f64) -> u8 {
        let mut rand = 0.0;
        let initial_zoom = zoom_factor;

        while zoom_factor >= 1.0 {
            rand += (self.smooth_noise(x, y, zoom_factor) as f64) * zoom_factor/2.0;
            zoom_factor /= 2.0;
        }

        (rand / initial_zoom) as u8
    }
}
