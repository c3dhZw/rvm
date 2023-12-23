use crate::memory::{read, write};
use crate::register::{reg, reg_r, update_flag, Register};
use crate::trap::trap;

const OP_COUNT: usize = 16;

fn opc(i: u16) -> usize {
  (i >> 12) as usize
}

#[inline]
fn dr(i: u16) -> u16 {
  (i >> 9) & 0x7
}

#[inline]
fn sr1(i: u16) -> u16 {
  (i >> 6) & 0x7
}

#[inline]
fn offset_6(i: u16) -> u16 {
  sext(i & 0x3F, 6)
}

#[inline]
fn offset_9(i: u16) -> u16 {
  sext(i & 0x1FF, 9)
}

#[inline]
fn offset_11(i: u16) -> u16 {
  sext(i & 0x7FF, 11)
}

#[inline]
fn base_r(i: u16) -> u16 {
  (i >> 6) & 0x7
}

#[inline]
fn op_br(i: u16) {
  let cond = *reg_r(Register::Cond);

  if (cond & (i >> 9 & 0x7)) != 0 {
    *reg_r(Register::Pc) += offset_9(i);
  }
}

#[inline]
fn op_add(i: u16) {
  let dr = dr(i);
  let sr1 = sr1(i);
  let sr2 = (i) & 0x7;
  let bit_5 = (i >> 5) & 0x1 == 1;

  *reg(dr) = {
    if bit_5 {
      *reg(sr1) + sext(i & 0x1F, 5)
    } else {
      *reg(sr1) + *reg(sr2)
    }
  };

  update_flag(dr.into());
}

#[inline]
fn op_st(i: u16) {
  write(*reg_r(Register::Pc) + offset_9(i), *reg(dr(i)));
}

#[inline]
fn op_jsr(i: u16) {
  *reg_r(Register::R7) = *reg_r(Register::Pc);
  let bit_11 = (i >> 11) & 0x1 == 1;

  *reg_r(Register::Pc) = if bit_11 {
    *reg_r(Register::Pc) + offset_11(i)
  } else {
    *reg(base_r(i))
  };
}

#[inline]
fn op_and(i: u16) {
  let dr = dr(i);
  let sr1 = sr1(i);
  let sr2 = (i) & 0x7;
  let bit_5 = (i >> 5) & 0x1 == 1;

  *reg(dr) = {
    if bit_5 {
      *reg(sr1) & sext(i & 0x1F, 5)
    } else {
      *reg(sr1) & *reg(sr2)
    }
  };

  update_flag(dr.into());
}

#[inline]
fn op_str(i: u16) {
  write(*reg(sr1(i)) + offset_6(i), *reg(dr(i)));
}

fn op_rti(_i: u16) {}

#[inline]
fn op_not(i: u16) {
  let dr = dr(i);
  let sr = sr1(i);

  *reg(dr) = !*reg(sr);

  update_flag(dr.into());
}

#[inline]
fn op_ld(i: u16) {
  let offset = offset_9(i);
  let dr = dr(i);

  *reg(dr) = read(*reg_r(Register::Pc) + offset);

  update_flag(dr.into());
}

#[inline]
fn op_ldi(i: u16) {
  let offset = offset_9(i);
  let dr = dr(i);

  *reg(dr) = read(read(*reg_r(Register::Pc) + offset));

  update_flag(dr.into());
}

#[inline]
fn op_ldr(i: u16) {
  let offset = offset_6(i);
  let dr = dr(i);
  let sr = sr1(i);

  *reg(dr) = read(*reg(sr) + offset);

  update_flag(dr.into());
}

#[inline]
fn op_sti(i: u16) {
  write(read(*reg_r(Register::Pc) + offset_9(i)), *reg(dr(i)));
}

#[inline]
fn op_jmp(i: u16) {
  let base_r = base_r(i);

  *reg_r(Register::Pc) = *reg(base_r);
}

fn op_res(_i: u16) {}

#[inline]
fn op_lea(i: u16) {
  let dr = dr(i);

  *reg(dr) = *reg_r(Register::Pc) + sext(i & 0x1FF, 9);

  update_flag(dr.into());
}

fn op_trap(i: u16) {
  trap(i);
}

static OPS: [fn(u16); OP_COUNT] = [
  op_br, op_add, op_ld, op_st, op_jsr, op_and, op_ldr, op_str, op_rti, op_not, op_ldi, op_sti,
  op_jmp, op_res, op_lea, op_trap,
];

pub static OP_NAMES: [&str; OP_COUNT] = [
  "br", "add", "ld", "st", "jsr", "and", "ldr", "str", "rti", "not", "ldi", "sti", "jmp", "res",
  "lea", "trap",
];

#[inline]
fn sext(x: u16, bit_count: u16) -> u16 {
  if (x >> (bit_count - 1)) & 1 == 1 {
    x | (0xFFFF << bit_count)
  } else {
    x
  }
}

pub fn op(i: u16) {
  OPS[opc(i)](i);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_opc() {
    assert_eq!(opc(0b0000_0000_0000_0000), 0);
    assert_eq!(opc(0b0001_0000_0000_0000), 1);
    assert_eq!(opc(0b0010_0000_0000_0000), 2);
    assert_eq!(opc(0b0011_0000_0000_0000), 3);
    assert_eq!(opc(0b0100_0000_0000_0000), 4);
    assert_eq!(opc(0b0101_0000_0000_0000), 5);
    assert_eq!(opc(0b0110_0000_0000_0000), 6);
    assert_eq!(opc(0b0111_0000_0000_0000), 7);
    assert_eq!(opc(0b1000_0000_0000_0000), 8);
    assert_eq!(opc(0b1001_0000_0000_0000), 9);
    assert_eq!(opc(0b1010_0000_0000_0000), 10);
    assert_eq!(opc(0b1011_0000_0000_0000), 11);
    assert_eq!(opc(0b1100_0000_0000_0000), 12);
    assert_eq!(opc(0b1101_0000_0000_0000), 13);
    assert_eq!(opc(0b1110_0000_0000_0000), 14);
    assert_eq!(opc(0b1111_0000_0000_0000), 15);

    assert_eq!(opc(0xF026), 15);
    assert_eq!(opc(0x1200), 1);
    assert_eq!(opc(0x1240), 1);
    assert_eq!(opc(0x1040), 1);
    assert_eq!(opc(0xF027), 15);
    assert_eq!(opc(0xF025), 15);
  }

  fn assert_fn_eq(f: fn(u16), g: fn(u16)) {
    assert_eq!(f as usize, g as usize);
  }

  #[test]
  fn test_ops() {
    assert_fn_eq(OPS[0], op_br);
    assert_fn_eq(OPS[1], op_add);
    assert_fn_eq(OPS[2], op_ld);
    assert_fn_eq(OPS[3], op_st);
    assert_fn_eq(OPS[4], op_jsr);
    assert_fn_eq(OPS[5], op_and);
    assert_fn_eq(OPS[6], op_ldr);
    assert_fn_eq(OPS[7], op_str);
    assert_fn_eq(OPS[8], op_rti);
    assert_fn_eq(OPS[9], op_not);
    assert_fn_eq(OPS[10], op_ldi);
    assert_fn_eq(OPS[11], op_sti);
    assert_fn_eq(OPS[12], op_jmp);
    assert_fn_eq(OPS[13], op_res);
    assert_fn_eq(OPS[14], op_lea);
    assert_fn_eq(OPS[15], op_trap);
  }

  #[test]
  fn test_op_names() {
    assert_eq!(OP_NAMES[0], "br");
    assert_eq!(OP_NAMES[1], "add");
    assert_eq!(OP_NAMES[2], "ld");
    assert_eq!(OP_NAMES[3], "st");
    assert_eq!(OP_NAMES[4], "jsr");
    assert_eq!(OP_NAMES[5], "and");
    assert_eq!(OP_NAMES[6], "ldr");
    assert_eq!(OP_NAMES[7], "str");
    assert_eq!(OP_NAMES[8], "rti");
    assert_eq!(OP_NAMES[9], "not");
    assert_eq!(OP_NAMES[10], "ldi");
    assert_eq!(OP_NAMES[11], "sti");
    assert_eq!(OP_NAMES[12], "jmp");
    assert_eq!(OP_NAMES[13], "res");
    assert_eq!(OP_NAMES[14], "lea");
    assert_eq!(OP_NAMES[15], "trap");
  }
}
