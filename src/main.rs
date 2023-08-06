mod interpreter;
mod bus;
mod drivers;


use bus::Bus;
use interpreter::Interpreter;
use clap::{Arg, Command};

pub const START_MEM_LOC: usize = 0x200; // 512th index


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

    let file_path = matches.get_one::<String>("rom").expect("ROM file path is required!");
    let absolute_file_path =  match std::env::current_dir() {
        Ok(mut path) => {
            path.push(file_path);
            path
        },
        Err(err) => panic!("{}", err),
    };         
    let file_contents = std::fs::read(file_path).unwrap_or_default();

    if file_contents.is_empty() {
        eprintln!("error while reading rom: contents empty / malformed file path")
    }               

    let sdl_ctx = sdl2::init().unwrap();
   // let event_pump = sdl_ctx.event_pump().unwrap(); 
    let mut interpreter = Interpreter::new(Bus::new(&sdl_ctx));

    // Load ROM program to memory
    interpreter.load_program(&file_contents);

    // Initiate loop to execute instructions
}