mod canvas;
mod noise;
mod rand;
mod vec;

use std::f64::consts::PI;
use std::time::Duration;

use noise::Noise;
use palette::rgb::Rgb;
use palette::{FromColor, Hue, IntoColor, Lch, Srgb};
use rand::Rand;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use palette::Hsl;

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
                let noise_u8 = noise.turbulence(i, j, 16.0);
                
                canvas.set_draw_color(Color::RGB(noise_u8, noise_u8, noise_u8));
                let _ = canvas.draw_point(i, j);
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

pub fn marble() {
    let mut canvas = Canvas::instantiate(128*8, 128*8);
    let mut event_pump = canvas
        .event_pump()
        .expect("failed to instantiate event pump");

    const H: usize = 128;
    const W: usize = 128;
    const X_PERIOD: f64 = 0.5;
    const Y_PERIOD: f64 = 2.5;
    const TURBULENCE_POWER: f64 = 8.0;
    const TURBULENCE_ZOOM: f64 = 16.0;
    
    let noise: Noise<H, W> = Noise::new();


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

        for i in 0..128*8 {
            for j in 0..128*8 {

                let xy_val = (i as f64) * X_PERIOD / (W as f64) +
                                 (j as f64) * Y_PERIOD / (H as f64) +
                                 TURBULENCE_POWER * (noise.turbulence(i, j, TURBULENCE_ZOOM) as f64) / 256.0;

                let sine_val = 256.0 * (xy_val * PI).sin().abs();
                let sine_val = sine_val as u8;

                canvas.set_draw_color(Color::RGB(sine_val, sine_val, sine_val));
                let _ = canvas.draw_point(i, j);
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

pub fn wood() {
    let mut canvas = Canvas::instantiate(128*8, 128*8);
    let mut event_pump = canvas
        .event_pump()
        .expect("failed to instantiate event pump");

    const H: usize = 800;
    const W: usize = 800;
    const XY_PERIOD: f64 = 12.0;
    const TURBULENCE_POWER: f64 = 0.1;
    const TURBULENCE_ZOOM: f64 = 128.0;
    
    let noise: Noise<H, W> = Noise::new();


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

        for i in 0..128*8 {
            for j in 0..128*8 {

                let x_val = (i as f64 - W as f64 / 2.0) / (W as f64);
                let y_val = (j as f64 - H as f64 / 2.0) / (H as f64);
                let dist = (x_val.powi(2) + y_val.powi(2)).sqrt() +
                    TURBULENCE_POWER * noise.turbulence(i, j, TURBULENCE_ZOOM) as f64 / 256.0;

                let sine_val = 128.0 * (2.0 * XY_PERIOD as f64 * dist * PI).sin().abs();

                let r = 80 + (sine_val as u8);
                let g = 30 + (sine_val as u8);
                let b = 30;

                canvas.set_draw_color(Color::RGB(r, g, b));
                let _ = canvas.draw_point(i, j);
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

pub fn main() {
    // random_noise()
    // cloud()
    // marble()
    wood()
}
