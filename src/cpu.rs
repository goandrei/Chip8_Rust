use crate::keypad::Keypad;
use crate::display::Display;
use crate::fontset::{FONTSET, FONTSET_LOCATION};

use rand::Rng;
use sdl2::Sdl;
use std::io;
use std::io::prelude::*;
use std::fs::File;

pub struct Cpu {
    // RAM memory
    memory: [u8; 4096],
    // registers
    vp: [u8; 16],
    // program counter
    pc: usize,
    // index register
    i: usize,
    // delay timer register
    delay_timer: u8, 
    // sound timer register
    sound_timer: u8,
    // keypad
    keypad: Keypad,
    // display
    display: Display,
    // stack
    stack: [u16; 16],
    // stack pointer
    sp: usize,
    // current opcode
    opcode: u16,
}

#[allow(dead_code)]
impl Cpu {
    pub fn new(sdl_context: &Sdl) -> Cpu {
        Cpu {
            memory: [0u8; 4096],
            vp: [0u8; 16],
            pc: 0usize,
            i: 0usize,
            delay_timer: 0u8,
            sound_timer: 0u8,
            keypad: Keypad::new(sdl_context),
            display: Display::new(sdl_context), 
            stack: [0u16; 16],
            sp: 0usize,
            opcode: 0u16,
        }
    }

    pub fn load_game(&mut self, path: String) -> io::Result<()> {
        let mut file = File::open(path)?;
        let mut buffer: Vec<u8> = Vec::new();

        file.read_to_end(&mut buffer)?;

        for (index, buf) in buffer.iter().enumerate() {
            self.memory[index + 0x200] = *buf;
        }

        Ok(())
    }

    pub fn initialize(&mut self) -> Result<bool, String> {
        // system expects that the application will be loaded at 0x200
        self.pc = 0x200;
        self.sp = 0;
        self.i =  0;
        self.opcode = 0;
        
        // initialize display
        self.display.initialize();

        // clear
        self.clear();

        // set fontset
        for (index, font) in FONTSET.iter().enumerate() {
            self.memory[index + FONTSET_LOCATION] = *font;
        }

        // reset timers
        self.delay_timer = 0;
        self.sound_timer = 0;

        Ok(true)
    }

    pub fn emulate_cycle(&mut self) -> Result<bool, String> {

        // update pressed keys
        match self.keypad.update_pressed_keys() {
            Ok(true) => (),
            Ok(false) => return Ok(false),
            Err(_) => return Err(String::new()),
        };

        // fetch opcode -> fetch it from memory at pc address
        // take 2 bytes since each opcode is 16bites long
        self.opcode = ((self.memory[self.pc] as u16) << 8) | (self.memory[self.pc + 1] as u16);

        // execute opcode
        self.execute_opcode()?;

        // update program counter
        self.pc += 2;

        // update timer
        self.update_timers();

        Ok(true)
    }

    fn clear(&mut self) {
        // clear stack
        for elem in self.stack.iter_mut() {
            *elem = 0;
        }
        // clear registers
        for v in self.vp.iter_mut() {
            *v = 0;
        }
        // clear memory
        for mem in self.memory.iter_mut() {
            *mem = 0;
        }
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn execute_opcode(&mut self) -> Result<bool, String> {
        // use shifting to cast u16 to u4
        let op1 = (self.opcode & 0xF000) >> 12;
        let op2 = (self.opcode & 0x0F00) >> 8;
        let op3 = (self.opcode & 0x00F0) >> 4;
        let op4 = self.opcode & 0x000F;

        match (op1, op2, op3, op4) {
            (0x0, 0x0, 0xE, 0x0) => {
                return self.clear_screen();
            },
            (0x0, 0x0, 0xE, 0xE) => {
                return self.return_from_subroutine();
            },
            (0x1, _, _, _) => {
                let target_addr: u16 = self.opcode & 0x0FFF;
                return self.jump_to_address(target_addr);
            },
            (0x2, _, _, _) => {
                let target_addr: u16 = self.opcode & 0x0FFF;
                return self.call_subroutine(target_addr);
            }
            (0x3, x, _, _) => {
                let constant = self.opcode & 0x00FF;
                return self.if_equal_skip(x, constant);
            },
            (0x4, x, _, _) => {
                let constant = self.opcode & 0x00FF;
                return self.if_not_equal_skip(x, constant);
            },
            (0x5, x, y, _) => {
                return self.if_registers_equal_skip(x, y);
            },
            (0x6, x, _, _) => {
                let nn = 0x00FF & self.opcode;
                return self.assign_to_reg(x, nn);
            },
            (0x7, x, _, _) => {
                let nn = 0x00FF & self.opcode;
                return self.add_to_reg(x, nn);
            },
            (0x8, x, y, 0) => {
                return self.set_reg(x, y);
            },
            (0x8, x, y, 1) => {
                return self.bitwise_or(x, y);
            },
            (0x8, x, y, 2) => {
                return self.bitwise_and(x, y);
            },
            (0x8, x, y, 3) => {
                return self.bitwise_xor(x, y);
            },
            (0x8, x, y, 4) => {
                return self.reg_sum(x, y);
            },
            (0x8, x, y, 5) => {
                return self.reg_sub(x, y);
            },
            (0x8, x, _, 6) => {
                return self.right_shift(x);
            },
            (0x8, x, y, 7) => {
                return self.reverse_sub(x, y);
            },
            (0x8, x, _, 0xE) => {
                return self.left_shift(x);
            },
            (0x9, x, y, 0) => {
                return self.if_reg_not_eq_skip(x, y);
            },
            (0xA, _, _, _) => {
                let nnn = self.opcode & 0x0FFF;
                return self.set_index_register(nnn);
            },
            (0xB, _, _, _) => {
                let nnn = self.opcode & 0x0FFF;
                return self.jump_to_addr_plus_v0(nnn);
            },
            (0xC, x, _, _) => {
                let nn = self.opcode & 0x00FF;
                return self.random_number_xor(x, nn);
            },
            (0xD, x, y, n) => {
                return self.draw(x, y, n);
            },
            (0xE, x, 0x9, 0xE) => {
                return self.if_key_pressed_skip(x);
            },
            (0xE, x, 0xA, 0x1) => {
                return self.if_key_not_pressed_skip(x);
            },
            (0xF, x, 0x0, 0x7) => {
                return self.set_vx_to_delay(x);
            },
            (0xF, x, 0x0, 0xA) => {
                return self.wait_key_press(x);
            },
            (0xF, x, 0x1, 0x5) => {
                return self.set_delay_to_vx(x);
            },
            (0xF, x, 0x1, 0x8) => {
                return self.set_sound_to_vx(x);
            },
            (0xF, x, 0x1, 0xE) => {
                return self.add_vx_to_i(x);
            },
            (0xF, x, 0x2, 0x9) => {
                return self.set_i_to_sprite_addr(x);
            },
            (0xF, x, 0x3, 0x3) => {
                return self.bcd(x);
            },
            (0xF, x, 0x5, 0x5) => {
                return self.reg_dump(x);
            },
            (0xF, x, 0x6, 0x5) => {
                return self.reg_load(x);
            },
            (_, _, _, _) => return Err(String::from(format!("Unknown opcode : {}", self.opcode))),
        }
    }

    /// Opcode : 00E0
    /// 
    /// Explanation : Clears the screen 
    fn clear_screen(&mut self) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | Instruction = 0x00E0 | Explanation = clear screen", self.pc);
        self.display.clear_screen()
    }

    /// Opcode : 00EE
    /// 
    /// Explanation : Returns from a subroutine
    /// TODO : edge case when stack is underflowed
    fn return_from_subroutine(&mut self) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | Instruction = 0x00EE | Explanation = return actual sp = {}, calling point = {}", self.pc, self.sp, self.stack[self.sp]);

        // move to the calling point 
        self.pc = self.stack[self.sp] as usize;
        // pop from the stack
        if self.sp > 0 {
            self.sp -= 1;
        }

        Ok(true)
    }

    /// Opcode : 1NNN
    /// 
    /// Explanation : Jumps to address NNN
    /// TODO : edge case when jumping outside memory
    fn jump_to_address(&mut self, target_addr: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | Instruction = 0x1NNN | Explanation = jump to target addr , NNN = {}", self.pc, target_addr);
        // self.pc = target_addr - 2 + 2(added in emulation_cycle)
        self.pc = (target_addr - 2) as usize;
        Ok(true)
    }

    /// Opcode : 2NNN
    /// 
    /// Explanation : Calls subroutine NNN
    fn call_subroutine(&mut self, target_addr: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | Instruction = 0x2NNN | Explanation = call subroutine , NNN = {}", self.pc, target_addr);
        // we must update the stack with the new pc
        self.sp += 1;
        self.stack[self.sp] = self.pc as u16;
        // the pc will be incremented so we have to cancel that increment so the next instruction will be at target_addr
        self.pc = (target_addr - 2) as usize;

        Ok(true)
    }

    /// Opcode : 3XNN
    /// 
    /// Explanation : Skips the next instruction if vp[X(4bits index)] == NN(8bits constant)
    fn if_equal_skip(&mut self, reg_index: u16, constant: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | Instruction = 0x3NNN | Explanation = if {} == {} then skip next", self.pc, constant, self.vp[reg_index as usize]);
        if (constant as u8) == self.vp[reg_index as usize] {
            self.pc += 2;
        }
        Ok(true)
    }

    /// Opcode : 4XNN
    /// 
    /// Explanation : Skips the next instruction if vp[X(4bits index)] != NN(8bits constant)
    fn if_not_equal_skip(&mut self, reg_index: u16, constant: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | Instruction = 0x4NNN | Explanation = if {} != {} then skip next", self.pc, constant, self.vp[reg_index as usize]);
        if (constant as u8) != self.vp[reg_index as usize] {
            self.pc += 2;
        }
        Ok(true)
    }

    /// Opcode : 5XY0
    /// 
    /// Explanation : Skips the next instruction if vp[Y(4bits index)] != vp[Y(4bits index)]
    fn if_registers_equal_skip(&mut self, x: u16, y: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | Instruction = 0x5XY0 | Explanation = if {} == {} then skip next", self.pc, self.vp[x as usize], self.vp[y as usize]);
        if self.vp[x as usize] == self.vp[y as usize] {
            self.pc += 2;
        }
        Ok(true)
    }

    /// Opcode : 6XNN
    /// 
    /// Explanation : Sets v[X(4 bits index)] = NN(8bits constant)
    fn assign_to_reg(&mut self, x: u16, nn: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | Instruction = 0x6XNN | Explanation =  v[{}] = {}", self.pc, x, nn);
        self.vp[x as usize] = nn as u8;
        Ok(true)
    }

    /// Opcode : 7XNN
    /// 
    /// Explanation : Adds NN to v[X(4 bits index)]
    fn add_to_reg(&mut self, x: u16, nn: u16) -> Result<bool, String> {
        let x = x as usize;

        println!("ProgramCounter = 0x{:02x} | Instruction = 0x7XNN | Explanation = v[{}] += {}", self.pc, x, nn);

        let vp_x: u16 = self.vp[x] as u16;
        let result: u16 = vp_x + nn;
        self.vp[x] = result as u8;
        Ok(true)
    }

    /// Opcode : 8XY0
    /// 
    /// Explanation : v[X(4 bits)] = v[Y(4 bits)]
    fn set_reg(&mut self, x: u16, y: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0x8XY0 : v[{}] = v[{}] , v[y] = {}", self.pc, x, y, self.vp[y as usize]);
        self.vp[x as usize] = self.vp[y as usize];
        Ok(true)
    }

    /// Opcode : 8XY1
    /// 
    /// Explanation : v[X] = v[X] | v[Y]
    fn bitwise_or(&mut self,x: u16, y: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0x8XY1 : v[{}] |= v[{}]", self.pc, x, y);
        self.vp[x as usize] |= self.vp[y as usize];
        Ok(true)
    }

    /// Opcode : 8XY2
    /// 
    /// Explanation : v[X] = v[X] & v[Y]
    fn bitwise_and(&mut self, x: u16, y: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0x8XY2 : v[{}] &= v[{}]", self.pc, x, y);
        self.vp[x as usize] &= self.vp[y as usize];
        Ok(true)
    }

    /// Opcode : 8XY3
    /// 
    /// Explanation : v[X] = v[X] & v[Y]
    fn bitwise_xor(&mut self, x: u16, y: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0x8XY3 : v[{}] ^= v[{}]", self.pc, x, y);
        self.vp[x as usize] ^= self.vp[y as usize];
        Ok(true)
    }

    /// Opcode : 8XY4
    /// 
    /// Explanation : v[X] = v[X] + v[Y]
    fn reg_sum(&mut self, x: u16, y: u16) -> Result<bool, String> {
        let vx = self.vp[x as usize] as u16;
        let vy = self.vp[y as usize] as u16;
        let result: u16 = vx + vy;
        // check if there was an overflow
        if result > 0xFF {
            self.vp[0xF] = 1u8;
        } else {
            self.vp[0xF] = 0u8;
        }
        // cast the result. If overflow occured the result will be trimmed
        self.vp[x as usize] = result as u8;
        println!("ProgramCounter = 0x{:02x} | 0x8XY4 : v[{}] += v[{}] result {}", self.pc, x, y, result);

        Ok(true)
    }

    /// Opcode : 8XY5
    /// 
    /// Explanation : v[X] = v[X] - v[Y]
    fn reg_sub(&mut self, x: u16, y: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0x8XY5 : v[{}] -= v[{}]", self.pc, x, y);
        let vx = self.vp[x as usize] as i16;
        let vy = self.vp[y as usize] as i16;
        let result: i16 = vx - vy;
        // check if there was an underflow
        if result < 0 {
            self.vp[0xF] = 0u8;
        } else {
            self.vp[0xF] = 1u8;
        }
        // cast the result. If overflow occured the result will be trimmed
        self.vp[x as usize] = result as u8;
        
        Ok(true)
    }

    /// Opcode : 8XY6
    /// 
    /// Explanation : v[X] = v[X] >> 1
    fn right_shift(&mut self, x: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0x8XY6 : v[{}] >>= 1", self.pc, x);
        self.vp[0xF] = self.vp[x as usize] & 0x01;
        self.vp[x as usize] = self.vp[x as usize] / 2;

        Ok(true)
    }

    /// Opcode : 8XY7
    /// 
    /// Explanation : v[X] = v[Y] - v[X]
    fn reverse_sub(&mut self, x: u16, y: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0x8XY7 : v[{}] = v[{}] - v[{}]", self.pc, x, y, x);
        if self.vp[y as usize] > self.vp[x as usize] {
            self.vp[0xF] = 1u8;
        } else {
            self.vp[0xF] = 0u8;
        }

        let vx = self.vp[x as usize] as i16;
        let vy = self.vp[y as usize] as i16;
        let result = vy - vx;

        self.vp[x as usize] = result as u8;

        Ok(true)
    }

    /// Opcode : 8XYE
    /// 
    /// Explanation : v[X] = v[X] << 1
    fn left_shift(&mut self, x: u16) -> Result<bool, String> {
        let x = x as usize;

        println!("ProgramCounter = 0x{:02x} | 0x8XYE : v[{}] = {} => v[{}] <<= 1", self.pc, x, self.vp[x], x);
        self.vp[0xf] = self.vp[x] & 0x80;
        let vp_x = self.vp[x] as u16;
        let result: u16 = vp_x * 2u16;
        self.vp[x] = result as u8;

        Ok(true)
    }

    /// Opcode : 9XY0
    /// 
    /// Explanation : if v[X] != v[Y] skip next instruction
    fn if_reg_not_eq_skip(&mut self, x: u16, y: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0x9XY0 : if v[{}] != v[{}] then skip", self.pc, x, y);
        if self.vp[x as usize] != self.vp[y as usize] {
            self.pc += 2;
        }
        Ok(true)
    }

    /// Opcode : ANNN
    /// 
    /// Explanation : set index register to address NNN
    fn set_index_register(&mut self, opcode: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0xANNN : I = {}", self.pc, opcode);
        self.i = opcode as usize;
        Ok(true)
    }

    /// Opcode : BNNN
    /// 
    /// Explanation : jumps to address NNN plus V[0]
    fn jump_to_addr_plus_v0(&mut self, nnn: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0xBNNN : pc = v[0] + {}", self.pc, nnn);
        self.pc = (self.vp[0x0] + (nnn as u8) - 2u8) as usize;
        Ok(true)
    }

    /// Opcode : CXNN
    /// 
    /// Explanation : v[X] = rand() & nn
    fn random_number_xor(&mut self, x: u16, nn: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0xCXNN : v[{}] = rand() ^ {}", self.pc, x, nn);
        let mut rng = rand::thread_rng();
        self.vp[x as usize] = rng.gen::<u8>() & (nn as u8);

        Ok(true)
    }

    /// Opcode : DXYN
    /// 
    /// Explanation : draws a sprite at (v[X], v[Y]) of size nx8
    ///               The values of the pixels are read from memory location I; I won't change after the execution
    ///               v[F] is set to 1 if any screen pixels are flipped from set to unset, 0 otherwise
    fn draw(&mut self, x: u16, y: u16, n: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0xDXYN : draw at ({}, {}) sprite 8x{}", self.pc, self.vp[x as usize], self.vp[y as usize], n);
        self.vp[0xF] = 0;
        //println!("I : {}", self.i);
        //println!("{} \n {} \n {} \n {} \n {} \n", self.memory[self.i+0], self.memory[self.i + 1], self.memory[self.i + 2], self.memory[self.i + 3], self.memory[self.i + 4]);

        let x = self.vp[x as usize] as u32;
        for byte in 0..n {
            let y = self.vp[y as usize] as u32 + byte as u32;
            let buff = self.memory[self.i + (byte as usize)];
            match self.display.draw(x, y, buff) {
                Ok(true) => self.vp[0xF] = 1,
                Ok(false) => (),
                Err(err) => return Err(err),
            };
        }

        Ok(true)
    }

    /// Opcode : EX9E
    /// 
    /// Explanation : if the key stored in v[X] is pressed skip next instruction
    fn if_key_pressed_skip(&mut self, x: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0xEX9E : if key {} is pressed skip inst", self.pc, self.vp[x as usize]);
        if self.keypad.is_key_pressed(self.vp[x as usize]) {
            self.pc += 2;
        }
        Ok(true)
    }

    /// Opcode : EXA1
    /// 
    /// Explanation : if the key stored in v[X] is not pressed skip next instruction
    fn if_key_not_pressed_skip(&mut self, x: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0xEXA1 : if key {} not pressed skip inst", self.pc, self.vp[x as usize]);
        if !self.keypad.is_key_pressed(self.vp[x as usize]) {
            self.pc += 2;
        }
        Ok(true)
    }

    /// Opcode : FX07
    /// 
    /// Explanation : v[X] = delay_timer
    fn set_vx_to_delay(&mut self, x: u16) -> Result<bool, String> {
        println!("0xFX07 : v[{}] = delay_timer = {}", x, self.delay_timer);
        self.vp[x as usize] = self.delay_timer;
        Ok(true)
    }

    /// Opcode : FX0A
    /// 
    /// Explanation : key press is awaited and stored in v[X]
    fn wait_key_press(&mut self, x: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0xFX0A : wait for key to be pressed", self.pc);
        self.vp[x as usize] = match self.keypad.wait_for_key() {
            Ok(0xFF) => return Ok(false),
            Ok(key) => key,
            Err(err) => return Err(err),
        };

        Ok(true)
    }

    /// Opcode : FX15
    /// 
    /// Explanantion : delay_timer = v[X]
    fn set_delay_to_vx(&mut self, x: u16) -> Result<bool, String> {
        println!("0xFX15 : delay_timer = v[{}] = {}", x, self.vp[x as usize]);
        self.delay_timer = self.vp[x as usize];
        Ok(true)
    }

    /// Opcode : FX18
    /// 
    /// Explanation : sound_timer = v[X]
    fn set_sound_to_vx(&mut self, x: u16) -> Result<bool, String> {
        println!("0xFX18 : sound_timer = v[{}] = {}", x, self.vp[x as usize]);
        self.sound_timer = self.vp[x as usize];
        Ok(true)
    }

    /// Opcode : FX1E
    /// 
    /// Explanation : I += v[X]
    fn add_vx_to_i(&mut self, x: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0xFX1E : I += v[{}]", self.pc, x);
        let i = self.i as u16;
        let vx = self.vp[x as usize] as u16;
        let result = i + vx;

        if result > 0xFFF {
            self.vp[0xF] = 1u8;
        } else {
            self.vp[0xF] = 0u8;
        }

        self.i = result as usize;
        Ok(true)
    }

    /// Opcode : FX29
    /// 
    /// Explanation : sets I to the location of the sprite for the character v[x]
    fn set_i_to_sprite_addr(&mut self, x: u16) -> Result<bool, String> {
        /* ----------------------
        *  0      0x50, 0x64, ...
        *  ----------------------
        *          ^^ - starting addr for fontset
        *  Each character is represented by a 4x5 font so each character takes 40 bits. 
        *  We are iterating through memory with 5 bytes jumps
        */
        println!("ProgramCounter = 0x{:02x} | 0xFX29 : I = location of char {} = {}", self.pc, self.vp[x as usize], FONTSET_LOCATION + 5 * (self.vp[x as usize]) as usize);
        let index = FONTSET_LOCATION + 5 * (self.vp[x as usize] as usize);
        self.i = index as usize;
        Ok(true)
    }

    /// Opcode : FX33
    /// 
    /// Explanation : Stores the binary representation of v[X] at the address of I as follows :
    ///               *(I+0)=BCD(3);
    ///               *(I+1)=BCD(2);
    ///               *(I+2)=BCD(1); 
    fn bcd(&mut self, x: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0xFX33 : bcd", self.pc);
        self.memory[self.i]     = (self.vp[x as usize] / 100) as u8;
        self.memory[self.i + 1] = ((self.vp[x as usize] / 10) % 10) as u8;
        self.memory[self.i + 2] = (self.vp[x as usize] % 10) as u8;

        Ok(true)
    }

    /// Opcode : FX55
    /// 
    /// Explanation : Stores v[0] to v[x] in memory starting at address I
    fn reg_dump(&mut self, x: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0xFX55 : reg dump for {}", self.pc, x);
        for (index, v) in self.vp.iter().enumerate() {
            self.memory[self.i + index] = *v;
            if index as u16 == x {
                break;
            }
        }

        Ok(true)
    }

    /// Opcode : FX65
    /// 
    /// Explanation : Fills v[0] to v[0xF] with values from memory starting with I
    fn reg_load(&mut self, x: u16) -> Result<bool, String> {
        println!("ProgramCounter = 0x{:02x} | 0xFX65 : reg load for {}", self.pc, x);
        for (index, v) in self.vp.iter_mut().enumerate() {
            *v = self.memory[self.i + index];
            if index as u16 == x {
                break;
            }
        }

        Ok(true)
    }

    pub fn get_display(&self) -> [u8; 32 * 64] {
        self.display.get_display()
    }

    pub fn set_display(&mut self, display: [u8; 32 * 64]) {
        self.display.set_display(display);
    }

    pub fn get_keypad(&mut self) -> &mut Keypad {
        &mut self.keypad
    }

    pub fn push_stack(&mut self, new_pc: u16) {
        self.sp += 1;
        self.stack[self.sp] = new_pc;
    }

    pub fn top_stack(&self) -> u16 {
        self.stack[self.sp]
    }

    pub fn set_register(&mut self, index: usize, value: u8) {
        self.vp[index] = value;
    }

    pub fn get_register(&self, index: usize) -> u8 {
        self.vp[index]
    }
 }

#[cfg(test)]
#[path = ".\\cpu_test.rs"]
mod cpu_test;