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
pub const PC_START: u16 = 0x3000;

pub fn load_image(name: &str, offset: u16) {
  let mut file = File::open(name).unwrap();
  let mut buffer = Vec::new();

  file.read_to_end(&mut buffer).unwrap();

  let start = PC_START + offset;

  memory::load(buffer, start);
}

pub fn run(offset: u16) {
  assert!(offset < u16::MAX - PC_START);

  *reg_r(Register::Pc) = PC_START + offset;

  while unsafe { RUNNING } {
    let pc = *reg_r(Register::Pc);
    let i = read(pc);
    *reg_r(Register::Pc) = pc + 1;

    op(i);
  }
}
