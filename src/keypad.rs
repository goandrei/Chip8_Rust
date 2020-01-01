use sdl2::Sdl;
use sdl2::EventPump;
use sdl2::keyboard::{Keycode};
use sdl2::event::Event;

pub struct Keypad {
    event_pump: EventPump,
    pressed_keys: [bool; 16],
}

impl Keypad {
    pub fn new(sdl_context: &Sdl) -> Keypad {

        let event_pump = sdl_context.event_pump().unwrap();
        Keypad {
            event_pump: event_pump,
            pressed_keys: [false; 16],
        }
    }

    pub fn emulate_cycle(&mut self) -> Result<bool, ()> {
        
        self.pressed_keys = [false; 16];

        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Num1 => self.pressed_keys[0x1] = true,
                        Keycode::Num2 => self.pressed_keys[0x2] = true,
                        Keycode::Num3 => self.pressed_keys[0x3] = true,
                        Keycode::Num4 => self.pressed_keys[0xC] = true,
                        Keycode::Q => self.pressed_keys[0x4] = true,
                        Keycode::W => self.pressed_keys[0x5] = true,
                        Keycode::E => self.pressed_keys[0x6] = true,
                        Keycode::R => self.pressed_keys[0xD] = true,
                        Keycode::A => self.pressed_keys[0x7] = true,
                        Keycode::S => self.pressed_keys[0x8] = true,
                        Keycode::D => self.pressed_keys[0x9] = true,
                        Keycode::F => self.pressed_keys[0xE] = true,
                        Keycode::Z => self.pressed_keys[0xA] = true,
                        Keycode::X => self.pressed_keys[0x0] = true,
                        Keycode::C => self.pressed_keys[0xB] = true,
                        Keycode::V => self.pressed_keys[0xF] = true,
                        _ => continue,
                    };
                },
                Event::Quit {..} => return Ok(false),
                _ => continue,
            };
        }


        let keys: Vec<Keycode> = self.event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        for key in keys {
            match key {
                Keycode::Num1 => self.pressed_keys[0x1] = true,
                Keycode::Num2 => self.pressed_keys[0x2] = true,
                Keycode::Num3 => self.pressed_keys[0x3] = true,
                Keycode::Num4 => self.pressed_keys[0xC] = true,
                Keycode::Q => self.pressed_keys[0x4] = true,
                Keycode::W => self.pressed_keys[0x5] = true,
                Keycode::E => self.pressed_keys[0x6] = true,
                Keycode::R => self.pressed_keys[0xD] = true,
                Keycode::A => self.pressed_keys[0x7] = true,
                Keycode::S => self.pressed_keys[0x8] = true,
                Keycode::D => self.pressed_keys[0x9] = true,
                Keycode::F => self.pressed_keys[0xE] = true,
                Keycode::Z => self.pressed_keys[0xA] = true,
                Keycode::X => self.pressed_keys[0x0] = true,
                Keycode::C => self.pressed_keys[0xB] = true,
                Keycode::V => self.pressed_keys[0xF] = true,
                _ => continue,
            }
        }

        Ok(true)     
    }

    pub fn wait_for_key(&mut self) -> Result<u8, String> {

        loop {
            // clone the old keyboard state
            let old_pressed_keys = self.pressed_keys.clone();

            // get the new state
            match self.emulate_cycle() {
                Ok(true) => (),
                Ok(false) => return Ok(0xFF),
                Err(_) => return Err(String::new()),
            };

            // finding a difference in the two arrays means there was a key press
            // or release -> in this case we don't return anything 
            for i in 0..16 {
                if old_pressed_keys[i] == false && old_pressed_keys[i] != self.pressed_keys[i] {
                    return Ok(i as u8);
                }
            }
        }
        
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.pressed_keys[key as usize]
    }

}