use crate::{bus::Bus, START_MEM_LOC, drivers::display::{DISPLAY_WIDTH, DISPLAY_HEIGHT}};



const MAX_STACK_SIZE: usize = 16;
const REGISTER_NUM: usize = 16;

// Max ram size - 512 reserved by the interpreter
const MAX_PROG_BYTES: usize = 3584;

pub struct Interpreter { 
    sp: usize,
    stack: [u16; MAX_STACK_SIZE], // 16 16-bit values
    pc: u16,
    pub bus: Bus,
    v: [u8; REGISTER_NUM], // V0 -> VF
    i: u16,
    dt: u8,
    st: u8,
}

enum ProgramCounterAction {
    Jump(u16),
    Next,
    Halt,
    NoOp,
}


impl Interpreter {
    pub fn new(bus: Bus) -> Self {
        Self { 
            sp: 0,
            stack: [0; MAX_STACK_SIZE],
            pc: START_MEM_LOC as u16,
            bus, 
            v: [0; REGISTER_NUM], 
            i: 0, 
            dt: 60, 
            st: 60, 
        }
    }

    pub fn load_program(&mut self, rom_bytes: &[u8]) -> () {
        let max_prog_idx = START_MEM_LOC + rom_bytes.len();
        self.bus.mem[START_MEM_LOC..max_prog_idx].copy_from_slice(rom_bytes)
    } 

    // @todo: work on macro_rules! for execution of instruction
    // @todo: handle display instructions
    pub fn execute(&mut self) {
        let opcode = self.bus.get_next_byte(self.pc as usize);
        let nibbles = (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        );
        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let ident = nibbles.0 as usize;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;

        let vx = self.v[x];
        let vy = self.v[y];

        let inc = match ident {
            0 => {
                match n {
                    0 => {
                        self.bus.display.clear_display();
                        ProgramCounterAction::Next
                    }
                    0xE => {
                        self.sp -=1;
                        let new_pc_val = self.stack[self.sp];
                        let pc_delta = self.pc - new_pc_val;
                        ProgramCounterAction::Jump(pc_delta)
                    }
                    _ => {
                        ProgramCounterAction::Halt
                    }
                }
            }
            1 => {
                self.pc = nnn as u16;
                ProgramCounterAction::NoOp
            }
            2 => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = nnn as u16;
                ProgramCounterAction::NoOp
            }
            3 => {
                if self.v[x] == kk {
                    ProgramCounterAction::Jump(4)
                } else {
                    ProgramCounterAction::Next
                }
            }
            4 => {
                if self.v[x] != kk {
                    ProgramCounterAction::Jump(4)
                } else {
                    ProgramCounterAction::Next
                }
            }
            5 => {
                if self.v[x] == self.v[y] {
                    ProgramCounterAction::Jump(4)
                } else {
                    ProgramCounterAction::Next
                }
            }
            6 => {
                self.v[x] = kk;
                ProgramCounterAction::Next
            }
            7 => {
                let vx = self.v[x];
                self.v[x] = vx + kk;
                ProgramCounterAction::Next
            }
            8 => {
                match n {
                    0 => self.v[x] = self.v[y],
                    1 => self.v[x] = vx | vy,
                    2 => self.v[x] = vx & vy,
                    3 => self.v[x] = vx ^ vy,
                    4 => {
                        let res = (vx as u16) + (vy as u16);
                        self.v[0xF] = if res > 0xFF {
                            1
                        } else {
                            0
                        };
                        // LSB gets kept during cast
                        self.v[x] = res as u8;
                    }
                    5 => {
                        self.v[0xF] = if vx > vy { 1 } else { 0 };
                        self.v[x] = vx - vy;
                    }
                    6 => {
                        self.v[0xF] = vx & 1;
                        self.v[x] = vx >> 1;
                    }
                    7 => {
                        self.v[0xF] = if vy > vx { 1 } else { 0 };
                        self.v[x] = vy - vx;
                    }
                    0xE => {
                        self.v[0xF] = vx & 1;
                        self.v[x] = vx << 1;
                    }
                    _ => {}

                }
                ProgramCounterAction::Next
            }
            9 => if vx != vy {
                ProgramCounterAction::Jump(4)
            } else {
                ProgramCounterAction::Next
            },
            0xA => {
                self.i = nnn as u16;
                ProgramCounterAction::Next
            },
            0xB => {
                let nnn_u16 = nnn as u16;
                let v0_u16 = self.v[0] as u16;
                ProgramCounterAction::Jump(nnn_u16 + v0_u16)
            },
            0xC => {
                let rand_u8 = rand::random::<u8>();
                self.v[x] = rand_u8 & kk;
                ProgramCounterAction::Next
            }
            // @todo
            0xD => {
                // between 0 and 16 8 bit wide rows
                for row in 0..n {
                    let row_addr = self.i + row as u16;
                    let sprite_row_data = self.bus.mem[row_addr as usize];

                    // 0 -> 8 columns
                    for column in 0..8 {
                        // ex. sprite_row_data 1100 0110
                        if sprite_row_data & (0b1000000 >> column) == 1 {
                            let adj_pixels_width = (vx+column) as usize % DISPLAY_WIDTH;
                            let adj_pixels_height = (vy+(row as u8)) as usize % DISPLAY_HEIGHT; 

                            let pixel_loc = adj_pixels_width + DISPLAY_WIDTH * adj_pixels_height;
                            self.v[0xF] = if self.bus.display.bits[pixel_loc] == true { 1 } else { 0 };
                            self.bus.display.bits[pixel_loc] ^= true;
                            self.bus.display.draw();
                        }
                    }
                }

                ProgramCounterAction::Next
            }
            0xE => {
                match y {
                    0x9 => {
                        match self.bus.is_key_pressed(vx as usize) {
                            Some(is_pressed) => if is_pressed { 
                                ProgramCounterAction::Jump(4)
                             } else { 
                                ProgramCounterAction::Next
                              },
                            None => {
                                ProgramCounterAction::Halt
                            }
                        }
                    }
                    0xA => {
                        match self.bus.is_key_pressed(vx as usize) {
                            Some(is_pressed) => if !is_pressed { 
                                ProgramCounterAction::Jump(4)
                             } else { 
                                ProgramCounterAction::Next
                            },
                            None => {
                                ProgramCounterAction::Halt
                            }
                        }
                    }
                    _ => {
                        ProgramCounterAction::Halt
                    }
                }
            }
            0xF => {
                match kk {
                    0x07 => {
                        self.v[x] = self.dt;
                        ProgramCounterAction::Next
                    },
                    // @todo: add wait with synchronisation rather than loop
                    0x0A => {
                        let mut any_pressed = false;
                        for i in 0..self.bus.keyboard.len() {
                            let pressed = self.bus.is_key_pressed(i);
                            if let Some(pressed) = Some(true) {
                                self.v[x] = self.bus.keyboard[i] as u8;
                                any_pressed = true;
                            }
                        }

                        if !any_pressed {
                            ProgramCounterAction::NoOp
                        } else {
                            ProgramCounterAction::Next
                        }
                        
                    }
                    0x15 => {
                        self.dt = vx;
                        ProgramCounterAction::Next
                    }
                    0x18 => {
                        self.st = vx;
                        ProgramCounterAction::Next
                    }
                    0x1E => {
                        let vx_u16 = self.v[x] as u16;
                        self.i = self.i + vx_u16;
                        ProgramCounterAction::Next
                    }
                    // @todo
                    0x29 => {ProgramCounterAction::NoOp},
                    0x33 => {ProgramCounterAction::NoOp},
                    0x55 => {ProgramCounterAction::NoOp},
                    0x65 => {ProgramCounterAction::NoOp},
                    _ => {ProgramCounterAction::Halt}
                }
            }
            _ => {
                ProgramCounterAction::Halt
            }
        };

        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
        }

        match inc {
            ProgramCounterAction::Jump(delta) => self.pc += delta,
            ProgramCounterAction::Next => self.pc += 2,
            ProgramCounterAction::NoOp => {},
            ProgramCounterAction::Halt => {
                // @todo: dbg + error handling
                eprintln!("")
            }
        }

        
    }
}