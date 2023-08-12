extern crate sdl2;

use sdl2::VideoSubsystem;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::render::Canvas;


pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
    canvas: Canvas<Window>,
    pub bits: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT]
}


impl Display { 
    pub fn new(video_ctx: VideoSubsystem) -> Self {
        let window = video_ctx.window("Chippy", DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        Self {
            canvas,
            bits: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT]
        }
    }


    pub fn draw(&self) {

    }

    pub fn clear_display(&mut self) {
        // @todo: Update buffer in window
        self.bits = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        self.canvas.clear();
    }
}