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
        let window = video_ctx.window("Chippy", 640,  320)
        .position_centered()
        .build()
        .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.clear();
        canvas.present();

        Self {
            canvas,
            bits: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT]
        }
    }


    pub fn draw(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();


        self.canvas.set_draw_color(Color::WHITE);
        for i in 0..self.bits.len() {
            if self.bits[i] {
                let x = (i as i32) % (DISPLAY_WIDTH as i32);
                let y = (i as i32) / (DISPLAY_WIDTH as i32);
                self.canvas.fill_rect(
                    Rect::new(
                        x*10,
                        y*10,
                        10,
                        10,
                    )
                ).unwrap();
            }
        }

       self.canvas.present();
    }

    pub fn clear_display(&mut self) {
        // @todo: Update buffer in window
        self.bits = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        self.canvas.clear();
    }
}