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

#[inline]
fn sext(x: u16, bit_count: u16) -> u16 {
  if (x >> (bit_count - 1)) & 1 == 1 {
    x | (0xFFFF << bit_count)
  } else {
    x
  }
}

static mut COUNT: u16 = 0;

pub fn op(i: u16) {
  unsafe {
    COUNT += 1;

    if COUNT > 30 {
      panic!("Too many instructions");
    }
  }
  println!("i: {:04x}, pc: {:04x}", i, *reg_r(Register::Pc));

  OPS[opc(i)](i);
}
