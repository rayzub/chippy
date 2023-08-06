use crate::{drivers::display::Display, START_MEM_LOC};
use sdl2::{Sdl, AudioSubsystem};
const MAX_MEM_SIZE: usize = 4096;

static FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0x90, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C 
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

// Controls keyboard actions, display actions and memory management with a bus structure.
pub struct Bus {
    pc: u16,
    pub mem: [u8; MAX_MEM_SIZE],
    pub keyboard: [bool; 16],
    pub display: Display,
    audio_ctx: AudioSubsystem,
}

impl Bus {
    pub fn new(ctx: &Sdl) -> Self {
        let mut mem = [0; MAX_MEM_SIZE];
        // Set reserved space for sprite fontset
        mem.copy_from_slice(&FONTSET);

        let video_ctx = ctx.video().unwrap();
        let audio_ctx = ctx.audio().unwrap();
        Self {
            pc: START_MEM_LOC as u16,
            mem,
            keyboard: [false; 16],
            display: Display::new(video_ctx),
            audio_ctx,
        }
    }
    
    pub fn get_next_byte(&self, pc: usize) -> u16 {
        // First byte, Second byte (instructions are 2 bytes long)
        (self.mem[pc] << 8 | self.mem[pc+1]) as u16
    }

    //pub fn update_ram(&self) {}
    pub fn toggle_key(&mut self, pos: usize) {
        let pos_val = self.keyboard[pos];
        self.keyboard[pos] = !pos_val;
    }
    pub fn is_key_pressed(&self, pos: usize) -> Option<bool> {
        if pos > 15 {
            return None
        }
        Some(self.keyboard[pos])
    }
}