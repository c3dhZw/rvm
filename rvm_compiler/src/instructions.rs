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
