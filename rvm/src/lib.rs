use std::fs::File;
use std::io::Read;

use memory::read;
use ops::op;
use register::{reg_r, Register};

pub mod memory;
pub mod ops;
pub mod register;
pub mod trap;

pub static mut RUNNING: bool = true;
pub static mut PC: u16 = 0;
pub static mut PC_START: u16 = 0;
// pub const PC_START: u16 = 0x3000;

pub fn load_image(name: &str, offset: u16) {
  let mut file = File::open(name).unwrap();
  let mut buffer = Vec::new();

  file.read_to_end(&mut buffer).unwrap();

  let header = u16::from_be_bytes([buffer[0], buffer[1]]);

  let buffer = &buffer[2..];

  // let start = PC_START + offset;
  let start = unsafe {
    PC_START = offset + header;

    PC_START
  };

  memory::load(buffer, start);
}

pub fn run(offset: u16) {
  let start = unsafe { PC_START };
  assert!(offset < u16::MAX - start);

  *reg_r(Register::Pc) = start + offset;

  unsafe {
    while RUNNING {
      PC = *reg_r(Register::Pc);

      let i = read(PC);
      *reg_r(Register::Pc) = PC + 1;

      op(i);
    }
  }
}
