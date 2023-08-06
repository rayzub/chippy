extern crate sdl2;

use sdl2::{video::Window, VideoSubsystem};

const DISPLAY_WIDTH: usize = 640;
const DISPLAY_HEIGHT: usize = 320;

pub struct Display {
    window: Window,
    bits: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT]
}


impl Display { 
    pub fn new(video_ctx: VideoSubsystem) -> Self {
        let window = video_ctx.window("Chippy", DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

        Self {
            window,
            bits: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT]
        }
    }

    pub fn clear_display(&mut self) {
        // @todo: Update buffer in window
        self.bits = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
    }
}