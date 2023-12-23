use chumsky::error::Simple;
use chumsky::primitive::{choice, just};
use chumsky::text::TextParser;
use chumsky::Parser;

use crate::registers::Register;

pub fn parse_register() -> impl Parser<char, Register, Error = Simple<char>> {
  choice((
    just("r0").map(|_| Register::R0),
    just("r1").map(|_| Register::R1),
    just("r2").map(|_| Register::R2),
    just("r3").map(|_| Register::R3),
    just("r4").map(|_| Register::R4),
    just("r5").map(|_| Register::R5),
    just("r6").map(|_| Register::R6),
    just("r7").map(|_| Register::R7),
  ))
  .padded()
}
