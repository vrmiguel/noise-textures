use itertools::Itertools;
use sdl2::{pixels::Color, render::WindowCanvas, EventPump, Sdl};

pub struct Canvas {
    sdl_context: Sdl,
    canvas: WindowCanvas,
}

impl Canvas {
    pub fn instantiate(width: u32, height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("caster", width, height)
            .resizable()
            .position_centered()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .build()
            .expect("failed to instantiate canvas");

        Self {
            sdl_context,
            canvas,
        }
    }

    pub fn debug_info(&self) {
        let version = sdl2::version::version();
        let info = "[INFO]";

        let video_subsystem = self.sdl_context.video().unwrap();

        println!("{} Using SDL {}", info, version);
        println!(
            "{} Available drivers: {}",
            info,
            sdl2::video::drivers().join(" ")
        );
        println!(
            "{} Current driver: {}",
            info,
            video_subsystem.current_video_driver()
        );
    }

    #[inline(always)]
    pub fn set_draw_color(&mut self, color: Color) {
        let _ = self.canvas.set_draw_color(color);
    }

    #[inline(always)]
    pub fn clear(&mut self, color: Color) {
        self.set_draw_color(color);
        self.canvas.clear();
    }

    #[inline(always)]
    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn event_pump(&self) -> Result<EventPump, String> {
        self.sdl_context.event_pump()
    }

    pub fn draw_point(&mut self, x: i32, y: i32) {
        let _ = self.canvas.draw_point((x, y));
    }
}
