mod interpreter;
mod bus;
mod drivers;


use bus::Bus;
use interpreter::Interpreter;
use clap::{Arg, Command};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub const START_MEM_LOC: usize = 0x200; // 512th index


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

    // Initiate loop to execute instructions
    'main: loop {
        // Execute single instruction
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } | Event::KeyUp { keycode, ..} => {
                    if keycode == Some(Keycode::Escape) {
                        break 'main
                    }

                    // Sets key on/off in keyboard array
                    interpreter.bus.toggle_key(0);
                },
                Event::Quit { .. } => { 
                    break 'main;
                }
                _ => {}
            }
        }

        interpreter.tick();
        interpreter.bus.display.draw();
    }
}
