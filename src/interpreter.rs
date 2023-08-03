use crate::bus::Bus;



const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const MAX_STACK_SIZE: usize = 16;
const REGISTER_NUM: usize = 16;

// Max ram size - 512 reserved by the interpreter
const MAX_PROG_BYTES: usize = 3584;
const START_MEM_LOC: usize = 0x200; // 512th index



macro_rules! execute {
    () => {};
}

pub struct Interpreter { 
    stack: [u16; MAX_STACK_SIZE], // 16 16-bit values
    pc: u16,
    bus: Bus,
    v: [u8; REGISTER_NUM], // V0 -> VF
    i: u16,
    dt: u8,
    st: u8,
}


impl Interpreter {
    pub fn new() -> Self {
        Self { 
            stack: [0; MAX_STACK_SIZE],
            pc: START_MEM_LOC as u16,
            bus: Bus::new(), 
            v: [0; REGISTER_NUM], 
            i: 0, 
            dt: 60, 
            st: 60, 
        }
    }

    pub fn load(&mut self, rom_bytes: &[u8]) -> () {
        let max_prog_idx = START_MEM_LOC + rom_bytes.len();
        self.bus.mem[START_MEM_LOC..max_prog_idx].copy_from_slice(rom_bytes)
    }

    


}