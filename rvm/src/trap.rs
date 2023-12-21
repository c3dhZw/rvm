use std::io::{stdin, Read};

use crate::memory::read;
use crate::register::reg;
use crate::RUNNING;

fn read_char() -> char {
  let mut buffer = String::with_capacity(1);

  unsafe {
    stdin().read_exact(buffer.as_mut_vec()).unwrap();
  }

  buffer.as_bytes()[0] as char
}

fn trap_get_char() {
  *reg(0x0) = read_char() as u16;
}

fn trap_out() {
  print!("{}", *reg(0x0) as u8 as char);
}

fn trap_puts() {
  let mut address = *reg(0x0);

  loop {
    let c = read(address) as u8;

    if c == 0 {
      break;
    }

    print!("{}", c as char);

    address += 1;
  }
}

fn trap_in() {
  trap_get_char();

  print!("{}", *reg(0x0) as u8 as char);
}

fn trap_putsp() {}

fn trap_halt() {
  unsafe {
    RUNNING = false;
  }
}

fn trap_in_u16() {
  let mut buffer = String::new();

  stdin().read_line(&mut buffer).unwrap();

  *reg(0x0) = buffer.trim().parse().unwrap();
}

fn trap_out_u16() {
  print!("{}", *reg(0x0));
}

static mut TRAPS: [fn(); 8] = [
  trap_get_char,
  trap_out,
  trap_puts,
  trap_in,
  trap_putsp,
  trap_halt,
  trap_in_u16,
  trap_out_u16,
];

pub fn trap(i: u16) {
  unsafe { TRAPS[((i & 0xFF) - 0x20) as usize]() }
}
