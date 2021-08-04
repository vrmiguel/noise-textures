mod canvas;
mod noise;
mod rand;
mod vec;

use std::convert::TryInto;
use std::time::Duration;

use noise::Noise;
use rand::Rand;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use crate::canvas::Canvas;

pub fn random_noise() {
    let mut canvas = Canvas::instantiate(128*8, 128*8);

    let mut noise: Noise<128, 128> = Noise::new();

    let mut event_pump = canvas
        .event_pump()
        .expect("failed to instantiate event pump");

    let rand = Rand::new();

    'running: loop {
        canvas.clear(Color::BLACK);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        noise.randomize();

        for i in 0..128*8 {
            for j in 0..128*8 {
                let noise_u8 = noise.smooth_noise(i, j);
                canvas.set_draw_color(Color::RGB(noise_u8, noise_u8, noise_u8));
                let _ = canvas.draw_point(i, j);
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

pub fn main() {
    random_noise()
}
