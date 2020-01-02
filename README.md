# Chip8_Rust

![alt text](https://github.com/goandrei/Chip8_Rust/blob/master/capture-20200102-162620.png)

This is a Chip8 emulator written in Rust. It uses [SDL2](https://github.com/Rust-SDL2/rust-sdl2) for the graphics.
The emulator doesn't have support for sounds and also it's running at your CPU's speed(the original Chip8 was running at a rate of 60Hz).
The emulator is not perfect, but I really learned a lot about how a CPU works and it was also my first "big" project in Rust.

## Usage
Clone the repo and run : 
> cargo run TICTAC

If no argument provided it will run TICTAC by default.

## Resources
I used the following resources during the development of my emulator:
- [Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
- [How to write an emulator (CHIP-8 interpreter)](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)
- [Cowgod's Chip-8](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)

## Credits
The games can be found here : [Zophar's Domain](https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html)
