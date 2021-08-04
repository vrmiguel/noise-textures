mod canvas;
mod rand;
mod vec;

use std::convert::TryInto;
use std::time::Duration;

use rand::Rand;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use crate::canvas::Canvas;

pub fn random_noise() {
    let mut canvas = Canvas::instantiate(200, 200);

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

        for i in 0..800 {
            for j in 0..800 {
                let noise = rand.rand_u8();
                canvas.set_draw_color(Color::RGB(noise, noise, noise));
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
