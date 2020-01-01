use super::*;
use sdl2;

#[test]
fn op_00e0_test() {
    let mut cpu = Cpu::new(&sdl2::init().unwrap());
    let new_display = [1; 32 * 64]; 
    cpu.set_display(new_display);

    assert!(new_display.iter().zip(cpu.get_display().iter()).all(|(a, b)| a == b), "Display was not set properly");
    
    cpu.opcode = 0x00E0;
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert!([0; 32 * 64].iter().zip(cpu.get_display().iter()).all(|(a, b)| a == b), "Opcode 00e0 did not clear screen");
}

#[test]
fn op_00ee_test() {
    const NEW_PC: u16 = 42;
    const PC: usize = 16;
    let mut cpu = Cpu::new(&sdl2::init().unwrap());
    cpu.push_stack(PC as u16);
    cpu.push_stack(NEW_PC);

    assert_eq!(NEW_PC, cpu.top_stack());

    cpu.opcode = 0x00EE;
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(PC as u16, cpu.top_stack());
    assert_eq!(NEW_PC as usize, cpu.pc);
}

#[test]
fn op_1nnn_test() {
    const TARGET_ADDR: u16 = 0x051A;
    let mut cpu = Cpu::new(&sdl2::init().unwrap());
    assert_eq!(0 as usize, cpu.pc);

    cpu.opcode = 0x1000 | TARGET_ADDR;
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(TARGET_ADDR as usize, cpu.pc);
}

#[test]
fn op_2nnn_test() {
    const TARGET_ADDR: u16 = 0x051A;
    let mut cpu = Cpu::new(&sdl2::init().unwrap());
    cpu.pc = 0x200;

    cpu.opcode = 0x2000 | TARGET_ADDR;
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(TARGET_ADDR as usize, cpu.pc);
    assert_eq!(0x200, cpu.stack[1]);
    assert_eq!(1 as usize, cpu.sp);
}

#[test]
fn op_3xnn_equal_test() {
    let x: usize = 3;
    let nn: u8 = 6; 
    let mut cpu = Cpu::new(&sdl2::init().unwrap());

    cpu.set_register(x, nn);
    assert_eq!(nn, cpu.get_register(x));
    assert_eq!(0, cpu.pc);

    cpu.opcode = 0x3000 | ((x as u16) << 8) | (nn as u16);
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(2, cpu.pc);
}

#[test]
fn op_3xnn_not_equal_test() {
    let x: usize = 3;
    let nn: u8 = 6; 
    let reg_nn: u8 = 15;
    let mut cpu = Cpu::new(&sdl2::init().unwrap());

    cpu.set_register(x, reg_nn);
    assert_eq!(reg_nn, cpu.get_register(x));
    assert_eq!(0, cpu.pc);

    cpu.opcode = 0x3000 | ((x as u16) << 8) | (nn as u16);
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(0, cpu.pc);
}

#[test]
fn op_4xnn_equal_test() {
    let x: usize = 3;
    let nn: u8 = 6; 
    let mut cpu = Cpu::new(&sdl2::init().unwrap());

    cpu.set_register(x, nn);
    assert_eq!(nn, cpu.get_register(x));
    assert_eq!(0, cpu.pc);

    cpu.opcode = 0x4000 | ((x as u16) << 8) | (nn as u16);
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(0, cpu.pc);
}

#[test]
fn op_4xnn_not_equal_test() {
    let x: usize = 3;
    let nn: u8 = 6; 
    let reg_nn: u8 = 15;
    let mut cpu = Cpu::new(&sdl2::init().unwrap());

    cpu.set_register(x, reg_nn);
    assert_eq!(reg_nn, cpu.get_register(x));
    assert_eq!(0, cpu.pc);

    cpu.opcode = 0x4000 | ((x as u16) << 8) | (nn as u16);
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(2, cpu.pc);
}

fn do_5xy0(x_val: u8, y_val: u8, expected_pc: usize) {
    let x: usize = 3;
    let y: usize = 5;
    let mut cpu = Cpu::new(&sdl2::init().unwrap());

    cpu.set_register(x, x_val);
    assert_eq!(x_val, cpu.get_register(x));
    cpu.set_register(y, y_val);
    assert_eq!(y_val, cpu.get_register(y));

    cpu.opcode = 0x5000 | ((x as u16) << 8) | ((y as u16) << 4);
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(expected_pc, cpu.pc);
}

#[test]
fn op_5xy0_equal_test() {
    do_5xy0(10, 10, 2);
}

#[test]
fn op_5xy0_not_equal_test() {
    do_5xy0(10, 12, 0);
}

#[test]
fn op_6xnn_test() {
    let x: usize = 5;
    let nn: u8 = 10;
    let mut cpu = Cpu::new(&sdl2::init().unwrap());
    assert_eq!(0u8, cpu.get_register(x));

    cpu.opcode = 0x6000 | ((x as u16) << 8) | (nn as u16);
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(nn, cpu.get_register(x));
}

#[test]
fn op_7xnn_test() {
    let x: usize = 5;
    let nn: u8 = 3;
    let mut cpu = Cpu::new(&sdl2::init().unwrap());
    assert_eq!(0u8, cpu.get_register(x));

    cpu.opcode = 0x7000 | ((x as u16) << 8) | (nn as u16);
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(nn, cpu.get_register(x));
}

#[test]
fn op_8xy0_test() {
    let x = 2u16;
    let y = 3u16;
    let val = 9u8;
    let mut cpu = Cpu::new(&sdl2::init().unwrap());

    assert_eq!(0u8, cpu.get_register(x as usize));
    assert_eq!(0u8, cpu.get_register(y as usize));
    
    cpu.set_register(y as usize, val);
    assert_eq!(val, cpu.get_register(y as usize));

    cpu.opcode = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4);
    let result = cpu.execute_opcode().unwrap();

    assert_eq!(true, result);
    assert_eq!(cpu.get_register(x as usize), cpu.get_register(y as usize));
}

#[test]
fn op_8xy_bitwise_op_test() {
    let x = 2u8;
    let y = 2u8;
    let val_x = 5u8;
    let val_y = 7u8;
    let mut cpu = Cpu::new(&sdl2::init().unwrap());

    cpu.set_register(x as usize, val_x);
    assert_eq!(val_x, cpu.get_register(x as usize));

    cpu.set_register(y as usize, val_y);
    assert_eq!(val_y, cpu.get_register(y as usize));

    cpu.opcode = 0x8001 | ((x as u16) << 8) | ((y as u16) << 4);
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(val_x | val_y, cpu.get_register(x as usize));

    let val_x = val_x | val_y;

    cpu.opcode = 0x8002 | ((x as u16) << 8) | ((y as u16) << 4);
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(val_x & val_y, cpu.get_register(x as usize));

    let val_x = val_x & val_y;

    cpu.opcode = 0x8003 | ((x as u16) << 8) | ((y as u16) << 4);
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(val_x ^ val_y, cpu.get_register(x as usize));
}

#[test]
fn op_8xy_reg_op_no_overflow_test() {
    let mut cpu = Cpu::new(&sdl2::init().unwrap());
    let x = 1u8;
    let y = 3u8;
    let val_x = 150;
    let val_y = 100;

    cpu.set_register(x as usize, val_x);
    assert_eq!(val_x, cpu.get_register(x as usize));
    cpu.set_register(y as usize, val_y);
    assert_eq!(val_y, cpu.get_register(y as usize));

    cpu.opcode = 0x8004 | ((x as u16) << 8) | ((y as u16) << 4);
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(val_x + val_y, cpu.get_register(x as usize));
    assert_eq!(0u8, cpu.get_register(0xF));

    let val_x = val_x + val_y;

    cpu.opcode = 0x8005 | ((x as u16) << 8) | ((y as u16) << 4);
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(val_x - val_y, cpu.get_register(x as usize));
    assert_eq!(1u8, cpu.get_register(0xF));
}

#[test]
fn op_8xy_reg_op_overflow_test() {
    let mut cpu = Cpu::new(&sdl2::init().unwrap());
    let x = 1u8;
    let y = 3u8;
    let val_x = 150i16;
    let val_y = 150i16;

    cpu.set_register(x as usize, val_x as u8);
    assert_eq!(val_x as u8, cpu.get_register(x as usize));
    cpu.set_register(y as usize, val_y as u8);
    assert_eq!(val_y as u8, cpu.get_register(y as usize));

    cpu.opcode = 0x8004 | ((x as u16) << 8) | ((y as u16) << 4);
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!((val_x + val_y) as u8, cpu.get_register(x as usize));
    assert_eq!(1u8, cpu.get_register(0xF));

    let val_x = (val_x + val_y) - 0xFF - 1;

    cpu.opcode = 0x8005 | ((x as u16) << 8) | ((y as u16) << 4);
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!((val_x - val_y) as u8, cpu.get_register(x as usize));
    assert_eq!(0u8, cpu.get_register(0xF));
}

#[test]
fn op_8xy7_reverse_sub_no_underflow_test() {
    let x = 3u8;
    let y = 5u8;
    let val_x = 55u8;
    let val_y = 65u8;
    let mut cpu = Cpu::new(&sdl2::init().unwrap());
    
    cpu.set_register(x as usize, val_x);
    assert_eq!(val_x, cpu.get_register(x as usize));
    cpu.set_register(y as usize, val_y);
    assert_eq!(val_y, cpu.get_register(y as usize));

    cpu.opcode = 0x8007 | ((x as u16) << 8) | ((y as u16) << 4);
    let result = cpu.execute_opcode().unwrap();

    assert_eq!(true, result);
    assert_eq!(val_y - val_x, cpu.get_register(x as usize));
    assert_eq!(1u8, cpu.get_register(0xF));
}

#[test]
fn op_8xy7_reverse_sub_underflow_test() {
    let x = 3u8;
    let y = 5u8;
    let val_x = 75i16;
    let val_y = 65i16;
    let mut cpu = Cpu::new(&sdl2::init().unwrap());
    
    cpu.set_register(x as usize, val_x as u8);
    assert_eq!(val_x as u8, cpu.get_register(x as usize));
    cpu.set_register(y as usize, val_y as u8);
    assert_eq!(val_y as u8, cpu.get_register(y as usize));

    cpu.opcode = 0x8007 | ((x as u16) << 8) | ((y as u16) << 4);
    let result = cpu.execute_opcode().unwrap();

    assert_eq!(true, result);
    assert_eq!((val_y - val_x) as u8, cpu.get_register(x as usize));
    assert_eq!(0u8, cpu.get_register(0xF));
}

#[test]
fn op_8xy4_6_shift_test() {
    let x = 3u8;
    let y = 5u8;
    let val_x = 53u8;
    let val_y = 10u8;
    let mut cpu = Cpu::new(&sdl2::init().unwrap());

    cpu.set_register(x as usize, val_x);
    assert_eq!(val_x, cpu.get_register(x as usize));
    cpu.set_register(y as usize, val_y);
    assert_eq!(val_y, cpu.get_register(y as usize));

    cpu.opcode = 0x8006 | ((x as u16) << 8);
    let result = cpu.execute_opcode().unwrap();

    assert_eq!(true, result);
    assert_eq!(val_x & 0x01, cpu.get_register(0xF));
    assert_eq!(val_x >> 1, cpu.get_register(x as usize));

    cpu.opcode = 0x800E | ((y as u16) << 8);
    let result = cpu.execute_opcode().unwrap();

    assert_eq!(true, result);
    assert_eq!(val_y & 0x80, cpu.get_register(0xF));
    assert_eq!(val_y << 1, cpu.get_register(y as usize));
}

#[test]
fn op_9xy0_test() {
    let x = 5u8;
    let y = 7u8;
    let val_x = 30u8;
    let val_y = 30u8;
    let new_val_y = 35u8;
    let mut cpu = Cpu::new(&sdl2::init().unwrap());

    cpu.set_register(x as usize, val_x);
    assert_eq!(val_x, cpu.get_register(x as usize));
    cpu.set_register(y as usize, val_y);
    assert_eq!(val_y, cpu.get_register(y as usize));

    cpu.opcode = 0x9000 | ((x as u16) << 8) | ((y as u16) << 4);
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(0usize, cpu.pc);

    cpu.set_register(y as usize, new_val_y);
    assert_eq!(new_val_y, cpu.get_register(y as usize));

    cpu.opcode = 0x9000 | ((x as u16) << 8) | ((y as u16) << 4);
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(2usize, cpu.pc);

}

#[test]
fn op_annn_test() {
    let mut cpu = Cpu::new(&sdl2::init().unwrap());
    cpu.opcode = 0xA523u16;

    assert_eq!(0usize, cpu.i);

    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!((cpu.opcode & 0x0FFF) as usize, cpu.i);
}

#[test]
fn op_bnnn_test() {
    let mut cpu = Cpu::new(&sdl2::init().unwrap());
    cpu.opcode = 0xB009u16;
    let val_vo = 5u16;

    assert_eq!(0usize, cpu.pc);
    cpu.set_register(0usize, val_vo as u8);
    assert_eq!(val_vo as u8, cpu.get_register(0usize));


    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(((cpu.opcode & 0x0FFF) + val_vo) as usize, cpu.pc);
}

#[test]
fn op_fx07_test() {
    let mut cpu = Cpu::new(&sdl2::init().unwrap());
    cpu.delay_timer = 15;

    cpu.opcode = 0xF307;
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(cpu.delay_timer, cpu.get_register(0x3 as usize));
}

#[test]
fn op_fx15_test() {
    let mut cpu = Cpu::new(&sdl2::init().unwrap());
    let x = 10u8;
    let val_x = 50u8;

    cpu.set_register(x as usize, val_x);
    assert_eq!(val_x, cpu.get_register(x as usize));

    cpu.opcode = 0xFA15;
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(val_x, cpu.delay_timer);
}

#[test]
fn op_fx18_test() {
    let mut cpu = Cpu::new(&sdl2::init().unwrap());
    let x = 10u8;
    let val_x = 50u8;

    cpu.set_register(x as usize, val_x);
    assert_eq!(val_x, cpu.get_register(x as usize));

    cpu.opcode = 0xFA18;
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(val_x, cpu.sound_timer);
}

#[test]
fn op_fx1e_test() {
    let x = 3u8;
    let val_x = 5u8;
    let mut cpu = Cpu::new(&sdl2::init().unwrap());
    assert_eq!(0usize, cpu.i);

    cpu.set_register(x as usize, val_x);
    assert_eq!(val_x, cpu.get_register(x as usize));

    cpu.opcode = 0xF31E;
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(val_x as usize, cpu.i);
    assert_eq!(0u8, cpu.get_register(0xF));
}

#[test]
fn op_fx33_test() {
    let mut cpu = Cpu::new(&sdl2::init().unwrap());
    let x = 3usize;
    let val_x = 152u8;
    cpu.set_register(x, val_x);

    cpu.opcode = 0xF333;
    let result = cpu.execute_opcode().unwrap();
    assert_eq!(true, result);
    assert_eq!(1, cpu.memory[cpu.i]);
    assert_eq!(5, cpu.memory[cpu.i + 1]);
    assert_eq!(2, cpu.memory[cpu.i + 2]);
}