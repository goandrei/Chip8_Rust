mod cpu;
mod keypad;
mod display;
mod fontset;

extern crate sdl2;
use cpu::Cpu;
use std::process;
use std::env;

fn get_rom_name(args: Vec<String>) -> String {
    if args.len() == 1 {
        return "TICTAC".to_string();
    } 

    args[1].clone()
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let rom = get_rom_name(args);

    let sdl_context = sdl2::init().unwrap();
    let mut cpu = Cpu::new(&sdl_context);

    match cpu.initialize() {
        Ok(_) => println!("Cpu initialized successfully!"),
        Err(err) => println!("Cpu init failed! : {}", err),
    }

    match cpu.load_game(format!("games/{}", rom)) {
        Ok(()) => println!("Game loaded!"),
        Err(err) =>  {
            println!("Could not load game! : {}", err);
            process::exit(1);
        },
    }
    
    loop {
        match cpu.emulate_cycle() {
            Ok(true) => continue,
            Ok(false) => process::exit(0),
            Err(err) => {
                println!("An error occured : {}", err);
                process::exit(1);
            },
        };
    }

}
