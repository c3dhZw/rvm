#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Register {
  R0 = 0x0,
  R1 = 0x1,
  R2 = 0x2,
  R3 = 0x3,
  R4 = 0x4,
  R5 = 0x5,
  R6 = 0x6,
  R7 = 0x7,
  Pc = 0x8,
  Cond = 0x9,
}

impl Register {
  pub fn bytecode(&self) -> u16 {
    *self as u16
  }
}
