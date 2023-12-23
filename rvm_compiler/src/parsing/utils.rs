use chumsky::error::Simple;
use chumsky::primitive::{just, take_until};
use chumsky::text::{int, newline, TextParser};
use chumsky::Parser;

pub fn parse_number() -> impl Parser<char, u16, Error = Simple<char>> {
  parse_hex().or(parse_decimal())
}

/// `x123a`
pub fn parse_hex() -> impl Parser<char, u16, Error = Simple<char>> {
  just("x").ignore_then(int(16)).try_map(|x: String, span| {
    u16::from_str_radix(&x, 16).map_err(|_| Simple::custom(span, "invalid hex number"))
  })
}

/// `#1234`
pub fn parse_decimal() -> impl Parser<char, u16, Error = Simple<char>> {
  just("#").ignore_then(int(10)).try_map(|x: String, span| {
    x.parse::<u16>()
      .map_err(|_| Simple::custom(span, "invalid decimal number"))
  })
}

/// ` , `
pub fn comma() -> impl Parser<char, char, Error = Simple<char>> {
  just(',').padded()
}

/// `; ... \n`
pub fn comment() -> impl Parser<char, (), Error = Simple<char>> {
  just(';').then(take_until(newline())).padded().ignored()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse_number() {
    assert_eq!(parse_number().parse("x1234"), Ok(0x1234));
    assert_eq!(parse_number().parse("#1234"), Ok(1234));
  }

  #[test]
  fn test_parse_hex() {
    assert_eq!(parse_hex().parse("x1234"), Ok(0x1234));
  }

  #[test]
  fn test_parse_decimal() {
    assert_eq!(parse_decimal().parse("#1234"), Ok(1234));
  }
}
