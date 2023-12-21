#[derive(Debug, Clone, Copy)]
pub enum Register {
  R0 = 0,
  R1,
  R2,
  R3,
  R4,
  R5,
  R6,
  R7,
  Pc,
  Cond,
  Count,
}

impl From<u16> for Register {
  fn from(r: u16) -> Self {
    match r {
      0 => Register::R0,
      1 => Register::R1,
      2 => Register::R2,
      3 => Register::R3,
      4 => Register::R4,
      5 => Register::R5,
      6 => Register::R6,
      7 => Register::R7,
      8 => Register::Pc,
      9 => Register::Cond,
      _ => panic!("Invalid register"),
    }
  }
}

impl From<Register> for usize {
  fn from(val: Register) -> Self {
    val as usize
  }
}

static mut REG: [u16; Register::Count as usize] = [0; Register::Count as usize];

#[inline]
pub fn reg_r(reg: Register) -> &'static mut u16 {
  unsafe { &mut REG[reg as usize] }
}

#[inline]
pub fn reg(reg: u16) -> &'static mut u16 {
  unsafe { &mut REG[reg as usize] }
}

const F_P: u16 = 1 << 0;
const F_Z: u16 = 1 << 1;
const F_N: u16 = 1 << 2;

#[inline]
pub fn update_flag(r: Register) {
  let r = *reg_r(r);

  *reg_r(Register::Cond) = if r == 0 {
    F_Z
  } else if r >> 15 == 1 {
    F_N
  } else {
    F_P
  };
}
