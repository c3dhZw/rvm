use chumsky::error::Simple;
use chumsky::primitive::{choice, just};
use chumsky::text::TextParser;
use chumsky::Parser;

use crate::instructions::{Instruction, TrapVect};
use crate::registers::parse_register;
use crate::utils::{comma, parse_number};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Op {
  BrNZ,
  BrNP,
  BrZP,
  BrN,
  BrZ,
  BrP,
  Add,
  Ld,
  St,
  Jsr,
  And,
  Ldr,
  Str,
  Rti,
  Not,
  Ldi,
  Sti,
  Jmp,
  Res,
  Lea,
  Trap,
}

/// BRNZ
/// ```
/// 0000 1 1 0 XXXXXXXXX
/// op   N Z P offset9
/// ```
///
/// BRNP
/// ```
/// 0000 1 0 1 XXXXXXXXX
/// op   N Z P offset9
/// ```
///
/// BRZP
/// ```
/// 0000 0 1 1 XXXXXXXXX
/// op   N Z P offset9
/// ```
///
/// BRN
/// ```
/// 0000 1 0 0 XXXXXXXXX
/// op   N Z P offset9
/// ```
///
/// BRZ
/// ```
/// 0000 0 1 0 XXXXXXXXX
/// op   N Z P offset9
/// ```
///
/// BRP
/// ```
/// 0000 0 0 1 XXXXXXXXX
/// op   N Z P offset9
/// ```
pub fn parse_br() -> impl Parser<char, Instruction, Error = Simple<char>> {
  choice((
    just("brnz").map(|_| Op::BrNZ),
    just("brnp").map(|_| Op::BrNP),
    just("brzp").map(|_| Op::BrZP),
    just("brn").map(|_| Op::BrN),
    just("brz").map(|_| Op::BrZ),
    just("brp").map(|_| Op::BrP),
  ))
  .padded()
  .then(parse_number())
  .map(|(br, offset)| match br {
    Op::BrNZ => Instruction::Br(true, true, false, offset as u8),
    Op::BrNP => Instruction::Br(true, false, true, offset as u8),
    Op::BrZP => Instruction::Br(false, true, true, offset as u8),
    Op::BrN => Instruction::Br(true, false, false, offset as u8),
    Op::BrZ => Instruction::Br(false, true, false, offset as u8),
    Op::BrP => Instruction::Br(false, false, true, offset as u8),
    _ => unreachable!(),
  })
}

/// ADD (register)
/// ```
/// 0001 XXX XXX 000 XXX
/// op   dr  sr1     sr2
/// ```
///
/// ADD (immediate)
/// ```
/// 0001 XXX XXX 1 XXXXX
/// op   dr  sr1   imm5
/// ```
///
/// If last arg is a register, bit 5 is 0.
pub fn parse_add() -> impl Parser<char, Instruction, Error = Simple<char>> {
  parse_add1().or(parse_add2())
}

/// ADD (register)
/// ```
/// 0001 XXX XXX 000 XXX
/// op   dr  sr1     sr2
/// ```
pub fn parse_add1() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("add")
    .padded()
    .ignore_then(parse_register().separated_by(comma()).exactly(3))
    .try_map(|args, span| {
      if args.len() != 3 {
        return Err(Simple::custom(span, "invalid add op"));
      }

      let dr = args[0];
      let sr1 = args[1];
      let sr2 = args[2];

      Ok(Instruction::Add1(dr, sr1, sr2))
    })
}

/// ADD (immediate)
/// ```
/// 0001 XXX XXX 1 XXXXX
/// op   dr  sr1   imm5
/// ```
pub fn parse_add2() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("add")
    .padded()
    .ignore_then(parse_register().separated_by(comma()).exactly(2))
    .then_ignore(comma())
    .then(parse_number())
    .try_map(|(registers, address), span| {
      if registers.len() != 2 {
        return Err(Simple::custom(span, "invalid add op"));
      }

      let dr = registers[0];
      let sr1 = registers[1];

      Ok(Instruction::Add2(dr, sr1, address as u8))
    })
}

/// LD
/// ```
/// 0010 XXX XXXXXXXXX
/// op   dr  offset9
/// ```
pub fn parse_ld() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("ld")
    .padded()
    .ignore_then(parse_register())
    .then_ignore(comma())
    .then(parse_number())
    .map(|(dr, address)| Instruction::Ld(dr, address as u8))
}

/// ST
/// ```
/// 0011 XXX XXXXXXXXX
/// op   sr  offset9
/// ```
pub fn parse_st() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("st")
    .padded()
    .ignore_then(parse_register())
    .then_ignore(comma())
    .then(parse_number())
    .map(|(sr, address)| Instruction::St(sr, address as u8))
}

/// JSR
/// ```
/// 0100 1 XXXXXXXXXXX
/// op     offset11
/// ```
pub fn parse_jsr() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("jsr")
    .padded()
    .ignore_then(parse_number())
    .map(|address| Instruction::Jsr(address as u8))
}

/// JSRR
/// ```
/// 0100 000 XXX    000000
/// op       base_r
/// ```
pub fn parse_jsrr() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("jsrr")
    .padded()
    .ignore_then(parse_register())
    .map(Instruction::Jsrr)
}

/// AND (register)
/// ```
/// 0101 XXX XXX 000 XXX
/// op   dr  sr1     sr2
/// ```
///
/// AND (immediate)
/// ```
/// 0101 XXX XXX 1 XXXXX
/// op   dr  sr1   imm5
/// ```
pub fn parse_and() -> impl Parser<char, Instruction, Error = Simple<char>> {
  parse_and1().or(parse_and2())
}

/// AND (register)
/// ```
/// 0101 XXX XXX 000 XXX
/// op   dr  sr1     sr2
/// ```
pub fn parse_and1() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("and")
    .padded()
    .ignore_then(parse_register().separated_by(comma()).exactly(3))
    .try_map(|args, span| {
      if args.len() != 3 {
        return Err(Simple::custom(span, "invalid and op"));
      }

      let dr = args[0];
      let sr1 = args[1];
      let sr2 = args[2];

      Ok(Instruction::And1(dr, sr1, sr2))
    })
}

/// AND (immediate)
/// ```
/// 0101 XXX XXX 1 XXXXX
/// op   dr  sr1   imm5
/// ```
pub fn parse_and2() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("and")
    .padded()
    .ignore_then(parse_register().separated_by(comma()).exactly(2))
    .then_ignore(comma())
    .then(parse_number())
    .try_map(|(registers, address), span| {
      if registers.len() != 2 {
        return Err(Simple::custom(span, "invalid and op"));
      }

      let dr = registers[0];
      let sr1 = registers[1];

      Ok(Instruction::And2(dr, sr1, address as u8))
    })
}

/// LDR
/// ```
/// 0110 XXX XXX    XXXXXX
/// op   dr  base_r offset6
/// ```
pub fn parse_ldr() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("ldr")
    .padded()
    .ignore_then(parse_register().separated_by(comma()).exactly(2))
    .then_ignore(comma())
    .then(parse_number())
    .try_map(|(args, address), span| {
      if args.len() != 2 {
        return Err(Simple::custom(span, "invalid ldr op"));
      }

      let dr = args[0];
      let base_r = args[1];

      Ok(Instruction::Ldr(dr, base_r, address as u8))
    })
}

/// STR
/// ```
/// 0111 XXX XXX    XXXXXX
/// op   dr  base_r offset6
/// ```
pub fn parse_str() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("str")
    .padded()
    .ignore_then(parse_register().separated_by(comma()).exactly(2))
    .then_ignore(comma())
    .then(parse_number())
    .try_map(|(args, address), span| {
      if args.len() != 2 {
        return Err(Simple::custom(span, "invalid str op"));
      }

      let dr = args[0];
      let base_r = args[1];

      Ok(Instruction::Str(dr, base_r, address as u8))
    })
}

/// RTI
/// ```
/// 1000 XXXXXXXXXXXX
/// op
/// ```
pub fn parse_rti() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("rti").padded().map(|_| Instruction::Rti)
}

/// NOT
/// ```
/// 1001 XXX XXX 111111
/// op   dr  sr1
/// ```
pub fn parse_not() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("not")
    .padded()
    .ignore_then(parse_register().separated_by(comma()).exactly(2))
    .try_map(|args, span| {
      if args.len() != 2 {
        return Err(Simple::custom(span, "invalid not op"));
      }

      let dr = args[0];
      let sr1 = args[1];

      Ok(Instruction::Not(dr, sr1))
    })
}

/// LDI
/// ```
/// 1010 XXX XXXXXXXXX
/// op   dr  offset9
/// ```
pub fn parse_ldi() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("ldi")
    .padded()
    .ignore_then(parse_register())
    .then_ignore(comma())
    .then(parse_number())
    .map(|(dr, address)| Instruction::Ldi(dr, address as u8))
}

/// STI
/// ```
/// 1011 XXX XXXXXXXXX
/// op   sr  offset9
/// ```
pub fn parse_sti() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("sti")
    .padded()
    .ignore_then(parse_register())
    .then_ignore(comma())
    .then(parse_number())
    .map(|(sr, address)| Instruction::Sti(sr, address as u8))
}

/// JMP
/// ```
/// 1100 000 XXX    000000
/// op       base_r
/// ```
pub fn parse_jmp() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("jmp")
    .padded()
    .ignore_then(parse_register())
    .map(Instruction::Jmp)
}

/// RES
/// ```
/// 1101 XXXXXXXXXXXX
/// op
/// ```
pub fn parse_res() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("res").padded().map(|_| Instruction::Res)
}

/// LEA
/// ```
/// 1110 XXX XXXXXXXXX
/// op   dr  offset9
/// ```
pub fn parse_lea() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("lea")
    .padded()
    .ignore_then(parse_register())
    .then_ignore(comma())
    .then(parse_number())
    .map(|(dr, address)| Instruction::Lea(dr, address as u8))
}

/// TRAP
/// ```
/// 1111 0000 XXXXXXXX
/// op        trap
/// ```
pub fn parse_trap() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("trap")
    .padded()
    .ignore_then(choice((
      // order of these is important
      // parsing will fail if `tin` is before any other starting with `tin`
      just("tinu16").map(|_| TrapVect::InU16),
      just("toutu16").map(|_| TrapVect::OutU16),
      just("tgetc").map(|_| TrapVect::GetC),
      just("toutc").map(|_| TrapVect::OutC),
      just("tputs").map(|_| TrapVect::PutS),
      just("thalt").map(|_| TrapVect::Halt),
      just("tin").map(|_| TrapVect::In),
    )))
    .map(Instruction::Trap)
}

/// HALT
/// alias for `trap thalt`
pub fn parse_halt() -> impl Parser<char, Instruction, Error = Simple<char>> {
  just("halt")
    .padded()
    .map(|_| Instruction::Trap(TrapVect::Halt))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::registers::Register;

  #[test]
  fn test_parse_br() {
    assert_eq!(
      parse_br().parse("brnz x12"),
      Ok(Instruction::Br(true, true, false, 0x12))
    );

    assert_eq!(
      parse_br().parse("brnp x12"),
      Ok(Instruction::Br(true, false, true, 0x12))
    );

    assert_eq!(
      parse_br().parse("brzp x12"),
      Ok(Instruction::Br(false, true, true, 0x12))
    );

    assert_eq!(
      parse_br().parse("brn x12"),
      Ok(Instruction::Br(true, false, false, 0x12))
    );

    assert_eq!(
      parse_br().parse("brz x12"),
      Ok(Instruction::Br(false, true, false, 0x12))
    );

    assert_eq!(
      parse_br().parse("brp x12"),
      Ok(Instruction::Br(false, false, true, 0x12))
    );

    assert_eq!(
      parse_br().parse("brnz #12"),
      Ok(Instruction::Br(true, true, false, 12))
    );

    assert_eq!(
      parse_br().parse("brnp #12"),
      Ok(Instruction::Br(true, false, true, 12))
    );

    assert_eq!(
      parse_br().parse("brzp #12"),
      Ok(Instruction::Br(false, true, true, 12))
    );

    assert_eq!(
      parse_br().parse("brn #12"),
      Ok(Instruction::Br(true, false, false, 12))
    );

    assert_eq!(
      parse_br().parse("brz #12"),
      Ok(Instruction::Br(false, true, false, 12))
    );

    assert_eq!(
      parse_br().parse("brp #12"),
      Ok(Instruction::Br(false, false, true, 12))
    );
  }

  #[test]
  fn test_parse_add() {
    assert_eq!(
      parse_add().parse("add r0, r1, r2"),
      Ok(Instruction::Add1(Register::R0, Register::R1, Register::R2))
    );

    assert_eq!(
      parse_add().parse("add r0, r1, #2"),
      Ok(Instruction::Add2(Register::R0, Register::R1, 2))
    );
    assert_eq!(
      parse_add().parse("add r0, r1, x2"),
      Ok(Instruction::Add2(Register::R0, Register::R1, 0x2))
    );
  }

  #[test]
  fn test_parse_add1() {
    assert_eq!(
      parse_add1().parse("add r0, r1, r2"),
      Ok(Instruction::Add1(Register::R0, Register::R1, Register::R2))
    );
  }

  #[test]
  fn test_parse_add2() {
    assert_eq!(
      parse_add2().parse("add r0, r1, #2"),
      Ok(Instruction::Add2(Register::R0, Register::R1, 2))
    );
    assert_eq!(
      parse_add2().parse("add r0, r1, x2"),
      Ok(Instruction::Add2(Register::R0, Register::R1, 0x2))
    );
  }

  #[test]
  fn test_parse_ld() {
    assert_eq!(
      parse_ld().parse("ld r0, #2"),
      Ok(Instruction::Ld(Register::R0, 2))
    );
    assert_eq!(
      parse_ld().parse("ld r0, x2"),
      Ok(Instruction::Ld(Register::R0, 0x2))
    );
  }

  #[test]
  fn test_parse_st() {
    assert_eq!(
      parse_st().parse("st r0, #2"),
      Ok(Instruction::St(Register::R0, 2))
    );
    assert_eq!(
      parse_st().parse("st r0, x2"),
      Ok(Instruction::St(Register::R0, 0x2))
    );
  }

  #[test]
  fn test_parse_jsr() {
    assert_eq!(parse_jsr().parse("jsr #2"), Ok(Instruction::Jsr(2)));
    assert_eq!(parse_jsr().parse("jsr x2"), Ok(Instruction::Jsr(0x2)));
  }

  #[test]
  fn test_parse_jsrr() {
    assert_eq!(
      parse_jsrr().parse("jsrr r0"),
      Ok(Instruction::Jsrr(Register::R0))
    );
  }

  #[test]
  fn test_parse_and() {
    assert_eq!(
      parse_and().parse("and r0, r1, r2"),
      Ok(Instruction::And1(Register::R0, Register::R1, Register::R2))
    );

    assert_eq!(
      parse_and().parse("and r0, r1, #2"),
      Ok(Instruction::And2(Register::R0, Register::R1, 2))
    );
    assert_eq!(
      parse_and().parse("and r0, r1, x2"),
      Ok(Instruction::And2(Register::R0, Register::R1, 0x2))
    );
  }

  #[test]
  fn test_parse_and1() {
    assert_eq!(
      parse_and1().parse("and r0, r1, r2"),
      Ok(Instruction::And1(Register::R0, Register::R1, Register::R2))
    );
  }

  #[test]
  fn test_parse_and2() {
    assert_eq!(
      parse_and2().parse("and r0, r1, #2"),
      Ok(Instruction::And2(Register::R0, Register::R1, 2))
    );
    assert_eq!(
      parse_and2().parse("and r0, r1, x2"),
      Ok(Instruction::And2(Register::R0, Register::R1, 0x2))
    );
  }

  #[test]
  fn test_parse_ldr() {
    assert_eq!(
      parse_ldr().parse("ldr r0, r1, #2"),
      Ok(Instruction::Ldr(Register::R0, Register::R1, 2))
    );
    assert_eq!(
      parse_ldr().parse("ldr r0, r1, x2"),
      Ok(Instruction::Ldr(Register::R0, Register::R1, 0x2))
    );
  }

  #[test]
  fn test_parse_str() {
    assert_eq!(
      parse_str().parse("str r0, r1, #2"),
      Ok(Instruction::Str(Register::R0, Register::R1, 2))
    );
    assert_eq!(
      parse_str().parse("str r0, r1, x2"),
      Ok(Instruction::Str(Register::R0, Register::R1, 0x2))
    );
  }

  #[test]
  fn test_parse_rti() {
    assert_eq!(parse_rti().parse("rti"), Ok(Instruction::Rti));
  }

  #[test]
  fn test_parse_not() {
    assert_eq!(
      parse_not().parse("not r0, r1"),
      Ok(Instruction::Not(Register::R0, Register::R1))
    );
  }

  #[test]
  fn test_parse_ldi() {
    assert_eq!(
      parse_ldi().parse("ldi r0, #2"),
      Ok(Instruction::Ldi(Register::R0, 2))
    );
    assert_eq!(
      parse_ldi().parse("ldi r0, x2"),
      Ok(Instruction::Ldi(Register::R0, 0x2))
    );
  }

  #[test]
  fn test_parse_sti() {
    assert_eq!(
      parse_sti().parse("sti r0, #2"),
      Ok(Instruction::Sti(Register::R0, 2))
    );
    assert_eq!(
      parse_sti().parse("sti r0, x2"),
      Ok(Instruction::Sti(Register::R0, 0x2))
    );
  }

  #[test]
  fn test_parse_jmp() {
    assert_eq!(
      parse_jmp().parse("jmp r0"),
      Ok(Instruction::Jmp(Register::R0))
    );
  }

  #[test]
  fn test_parse_res() {
    assert_eq!(parse_res().parse("res"), Ok(Instruction::Res));
  }

  #[test]
  fn test_parse_lea() {
    assert_eq!(
      parse_lea().parse("lea r0, #2"),
      Ok(Instruction::Lea(Register::R0, 2))
    );
    assert_eq!(
      parse_lea().parse("lea r0, x2"),
      Ok(Instruction::Lea(Register::R0, 0x2))
    );
  }
}
