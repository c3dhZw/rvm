use crate::registers::Register;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
  /// ```
  /// 0000 X X X XXXXXXXXX
  /// op   N Z P offset9
  /// ```
  Br(bool, bool, bool, u8),
  /// ```
  /// 0001 XXX XXX 000 XXX
  /// op   dr  sr1     sr2
  /// ```
  Add1(Register, Register, Register),
  /// ```
  /// 0001 XXX XXX 1 XXXXX
  /// op   dr  sr1   imm5
  /// ```
  Add2(Register, Register, u8),
  /// ```
  /// 0010 XXX XXXXXXXXX
  /// op   dr  offset9
  /// ```
  Ld(Register, u8),
  /// ```
  /// 0011 XXX XXXXXXXXX
  /// op   sr  offset9
  /// ```
  St(Register, u8),
  /// ```
  /// 0100 1 XXXXXXXXXXX
  /// op     offset11
  /// ```
  Jsr(u8),
  /// ```
  /// 0100 000 XXX    000000
  /// op       base_r
  /// ```
  Jsrr(Register),
  /// ```
  /// 0101 XXX XXX 000 XXX
  /// op   dr  sr1     sr2
  /// ```
  And1(Register, Register, Register),
  /// ```
  /// 0101 XXX XXX 1 XXXXX
  /// op   dr  sr1   imm5
  /// ```
  And2(Register, Register, u8),
  /// ```
  /// 0110 XXX XXX    XXXXXX
  /// op   dr  base_r offset6
  /// ```
  Ldr(Register, Register, u8),
  /// ```
  /// 0111 XXX XXX    XXXXXX
  /// op   dr  base_r offset6
  /// ```
  Str(Register, Register, u8),
  /// ```
  /// 1000 XXXXXXXXXXXX
  /// op
  /// ```
  Rti,
  /// ```
  /// 1001 XXX XXX 111111
  /// op   dr  sr1
  /// ```
  Not(Register, Register),
  /// ```
  /// 1010 XXX XXXXXXXXX
  /// op   dr  offset9
  /// ```
  Ldi(Register, u8),
  /// ```
  /// 1011 XXX XXXXXXXXX
  /// op   sr  offset9
  /// ```
  Sti(Register, u8),
  /// ```
  /// 1100 000 XXX    000000
  /// op       base_r
  /// ```
  Jmp(Register),
  /// ```
  /// 1101 XXXXXXXXXXXX
  /// op
  /// ```
  Res,
  /// ```
  /// 1110 XXX XXXXXXXXX
  /// op   dr  offset9
  /// ```
  Lea(Register, u8),
  /// ```
  /// 1111 0000 XXXXXXXX
  /// op        trap
  /// ```
  Trap(TrapVect),
}
impl Instruction {
  pub fn bytecode(&self) -> u16 {
    match self {
      Instruction::Br(n, z, p, offset9) => {
        let mut out = 0b0000_0000_0000_0000;
        if *n {
          out |= 0b0000_1000_0000_0000;
        }
        if *z {
          out |= 0b0000_0100_0000_0000;
        }
        if *p {
          out |= 0b0000_0010_0000_0000;
        }
        out |= u16::from(*offset9);
        out
      }
      Instruction::Add1(dr, sr1, sr2) => {
        let mut out = 0b0001_0000_0000_0000;
        out |= dr.bytecode() << 9;
        out |= sr1.bytecode() << 6;
        out |= sr2.bytecode();
        out
      }
      Instruction::Add2(dr, sr1, imm5) => {
        let mut out = 0b0001_0000_0000_0000;
        out |= dr.bytecode() << 9;
        out |= sr1.bytecode() << 6;
        out |= u16::from(*imm5);
        out
      }
      Instruction::Ld(dr, offset9) => {
        let mut out = 0b0010_0000_0000_0000;
        out |= dr.bytecode() << 9;
        out |= u16::from(*offset9);
        out
      }
      Instruction::St(sr, offset9) => {
        let mut out = 0b0011_0000_0000_0000;
        out |= sr.bytecode() << 9;
        out |= u16::from(*offset9);
        out
      }
      Instruction::Jsr(offset11) => {
        let mut out = 0b0100_1000_0000_0000;
        out |= u16::from(*offset11);
        out
      }
      Instruction::Jsrr(base_r) => {
        let mut out = 0b0100_0000_0000_0000;
        out |= base_r.bytecode() << 6;
        out
      }
      Instruction::And1(dr, sr1, sr2) => {
        let mut out = 0b0101_0000_0000_0000;
        out |= dr.bytecode() << 9;
        out |= sr1.bytecode() << 6;
        out |= sr2.bytecode();
        out
      }
      Instruction::And2(dr, sr1, imm5) => {
        let mut out = 0b0101_0000_0000_0000;
        out |= dr.bytecode() << 9;
        out |= sr1.bytecode() << 6;
        out |= u16::from(*imm5);
        out
      }
      Instruction::Ldr(dr, base_r, offset6) => {
        let mut out = 0b0110_0000_0000_0000;
        out |= dr.bytecode() << 9;
        out |= base_r.bytecode() << 6;
        out |= u16::from(*offset6);
        out
      }
      Instruction::Str(sr, base_r, offset6) => {
        let mut out = 0b0111_0000_0000_0000;
        out |= sr.bytecode() << 9;
        out |= base_r.bytecode() << 6;
        out |= u16::from(*offset6);
        out
      }
      Instruction::Rti => 0b1000_0000_0000_0000,
      Instruction::Not(dr, sr) => {
        let mut out = 0b1001_0000_0000_0000;
        out |= dr.bytecode() << 9;
        out |= sr.bytecode() << 6;
        out
      }
      Instruction::Ldi(dr, offset9) => {
        let mut out = 0b1010_0000_0000_0000;
        out |= dr.bytecode() << 9;
        out |= u16::from(*offset9);
        out
      }
      Instruction::Sti(sr, offset9) => {
        let mut out = 0b1011_0000_0000_0000;
        out |= sr.bytecode() << 9;
        out |= u16::from(*offset9);
        out
      }
      Instruction::Jmp(base_r) => {
        let mut out = 0b1100_0000_0000_0000;
        out |= base_r.bytecode() << 6;
        out
      }
      Instruction::Res => 0b1101_0000_0000_0000,
      Instruction::Lea(dr, offset9) => {
        let mut out = 0b1110_0000_0000_0000;
        out |= dr.bytecode() << 9;
        out |= u16::from(*offset9);
        out
      }
      Instruction::Trap(trap) => {
        let mut out = 0b1111_0000_0000_0000;
        out |= trap.bytecode();
        out
      }
    }
  }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TrapVect {
  GetC = 0x20,
  OutC = 0x21,
  PutS = 0x22,
  In = 0x23,
  PutSp = 0x24,
  Halt = 0x25,
  InU16 = 0x26,
  OutU16 = 0x27,
}

impl TrapVect {
  pub fn bytecode(&self) -> u16 {
    *self as u16
  }
}
