mod interpreter;
mod bus;
mod drivers;


use bus::Bus;
use interpreter::Interpreter;
use clap::{Arg, Command};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, SystemTime, Instant};

pub const START_MEM_LOC: usize = 0x200; // 512th index
const FRAME_RATE: Duration = Duration::from_millis(16);
const IX_RATE: Duration = Duration::from_millis(1);


pub struct Config { 
    scale_factor: u32,
    debugger: bool,
}

fn main() {
    let matches = Command::new("Chippy - Chip8 Emulator")
                           .version("0.1.0")
                           .author("Rayhan Zuberi (rayzub)")
                           .arg(Arg::new("rom")
                                .short('r')
                                .help("Specify the file containing ROM to run")
                                .required(true)
                            )
                            .get_matches();

    let file_path: &String = matches.get_one::<String>("rom").expect("ROM file path is required!");
    let absolute_file_path =  match std::env::current_dir() {
        Ok(mut path) => {
            path.push(file_path);
            path
        },
        Err(err) => panic!("{}", err),
    };         
    let file_contents = std::fs::read(absolute_file_path).unwrap_or_default();

    if file_contents.is_empty() {
        eprintln!("error while reading rom: contents empty / malformed file path")
    }               

    let sdl_ctx = sdl2::init().unwrap();
    let mut event_pump = sdl_ctx.event_pump().unwrap(); 
    let mut interpreter = Interpreter::new(Bus::new(&sdl_ctx));

    // Load ROM program to memory
    interpreter.load_program(&file_contents);
    let mut last_ix = Instant::now();
    let mut last_frame = Instant::now();
    // Initiate loop to execute instructions
    'main: loop {
        // Execute single instruction
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } | Event::KeyUp { keycode: Some(Keycode::Num1), .. } => interpreter.bus.toggle_key(0),
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } | Event::KeyUp { keycode: Some(Keycode::Num2), .. } => interpreter.bus.toggle_key(1),
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } | Event::KeyUp { keycode: Some(Keycode::Num3), .. } => interpreter.bus.toggle_key(2),
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } | Event::KeyUp { keycode: Some(Keycode::Num4), .. } => interpreter.bus.toggle_key(3),
                Event::KeyDown { keycode: Some(Keycode::Q), .. } | Event::KeyUp { keycode: Some(Keycode::Q), .. } => interpreter.bus.toggle_key(4),
                Event::KeyDown { keycode: Some(Keycode::W), .. } | Event::KeyUp { keycode: Some(Keycode::W), .. } => interpreter.bus.toggle_key(5),
                Event::KeyDown { keycode: Some(Keycode::E), .. } | Event::KeyUp { keycode: Some(Keycode::E), .. } => interpreter.bus.toggle_key(6),
                Event::KeyDown { keycode: Some(Keycode::R), .. } | Event::KeyUp { keycode: Some(Keycode::R), .. } => interpreter.bus.toggle_key(7),
                Event::KeyDown { keycode: Some(Keycode::A), .. } | Event::KeyUp { keycode: Some(Keycode::A), .. } => interpreter.bus.toggle_key(8),
                Event::KeyDown { keycode: Some(Keycode::S), .. } | Event::KeyUp { keycode: Some(Keycode::S), .. } => interpreter.bus.toggle_key(9),
                Event::KeyDown { keycode: Some(Keycode::D), .. } | Event::KeyUp { keycode: Some(Keycode::D), .. } => interpreter.bus.toggle_key(10),
                Event::KeyDown { keycode: Some(Keycode::F), .. } | Event::KeyUp { keycode: Some(Keycode::F), .. } => interpreter.bus.toggle_key(11),
                Event::KeyDown { keycode: Some(Keycode::Z), .. } | Event::KeyUp { keycode: Some(Keycode::Z), .. } => interpreter.bus.toggle_key(12),
                Event::KeyDown { keycode: Some(Keycode::X), .. } | Event::KeyUp { keycode: Some(Keycode::X), .. } => interpreter.bus.toggle_key(13),
                Event::KeyDown { keycode: Some(Keycode::C), .. } | Event::KeyUp { keycode: Some(Keycode::C), .. } => interpreter.bus.toggle_key(14),
                Event::KeyDown { keycode: Some(Keycode::V), .. } | Event::KeyUp { keycode: Some(Keycode::V), .. } => interpreter.bus.toggle_key(15),
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main,
                _ => {}
            }
        }

        if last_ix.elapsed() >= IX_RATE {
            interpreter.tick();
            last_ix = Instant::now();
        }

        if last_frame.elapsed() >= FRAME_RATE { 
            interpreter.bus.display.draw();
            last_frame = Instant::now();
        }
    }
}
