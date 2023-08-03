use crate::drivers::{display::Display, audio::Audio, keyboard::Keyboard};
const MAX_MEM_SIZE: usize = 4096;
// Controls keyboard actions, display actions and memory management with a bus structure.

pub struct Bus { 
    pub mem: [u8; MAX_MEM_SIZE],
    display: Display,
    audio: Audio,
    keyboard: Keyboard,

}

impl Bus {

    pub fn new() -> Self {
        Self {
            mem: [0; MAX_MEM_SIZE],
            display: Display {},
            audio: Audio {},
            keyboard: Keyboard {}
        }
    }
}