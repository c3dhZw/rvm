use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::error::Simple;
use chumsky::primitive::{choice, end};
use chumsky::text::{newline, TextParser};
use chumsky::Parser;
use instructions::Instruction;
use ops::{
  parse_add, parse_and, parse_br, parse_halt, parse_jmp, parse_jsr, parse_ld, parse_ldi, parse_ldr,
  parse_lea, parse_not, parse_res, parse_rti, parse_st, parse_sti, parse_str, parse_trap,
};
use utils::comment;

pub mod instructions;
pub mod ops;
pub mod registers;
pub mod utils;

#[derive(Debug, PartialEq)]
pub struct Program {
  pub instructions: Vec<Instruction>,
}

pub fn parse(input: &str) -> Result<Program, Vec<Report>> {
  match parse_program().parse(input) {
    Ok(program) => Ok(program),
    Err(errs) => Err(
      errs
        .into_iter()
        .map(|e| {
          let msg = if let chumsky::error::SimpleReason::Custom(msg) = e.reason() {
            msg.clone()
          } else {
            format!(
              "{}{}, expected {}",
              if e.found().is_some() {
                "Unexpected token"
              } else {
                "Unexpected end of input"
              },
              if let Some(label) = e.label() {
                format!(" while parsing {}", label)
              } else {
                String::new()
              },
              if e.expected().len() == 0 {
                "something else".to_string()
              } else {
                e.expected()
                  .map(|expected| match expected {
                    Some(expected) => expected.to_string(),
                    None => "end of input".to_string(),
                  })
                  .collect::<Vec<_>>()
                  .join(", ")
              },
            )
          };

          let report = Report::build(ReportKind::Error, (), e.span().start)
            .with_code(3)
            .with_message(msg)
            .with_label(
              Label::new(e.span())
                .with_message(match e.reason() {
                  chumsky::error::SimpleReason::Custom(msg) => msg.clone(),
                  _ => format!(
                    "Unexpected {}",
                    e.found()
                      .map(|c| format!("token {}", c.fg(Color::Red)))
                      .unwrap_or_else(|| "end of input".to_string())
                  ),
                })
                .with_color(Color::Red),
            );

          let report = match e.reason() {
            chumsky::error::SimpleReason::Unclosed { span, delimiter } => report.with_label(
              Label::new(span.clone())
                .with_message(format!(
                  "Unclosed delimiter {}",
                  delimiter.fg(Color::Yellow)
                ))
                .with_color(Color::Yellow),
            ),
            chumsky::error::SimpleReason::Unexpected => report,
            chumsky::error::SimpleReason::Custom(_) => report,
          };

          report.finish()
        })
        .collect(),
    ),
  }
}

pub fn print_errors(input: &str, errs: Vec<Report>) {
  for err in errs {
    err.eprint(Source::from(input)).unwrap();
  }
}

fn parse_program() -> impl Parser<char, Program, Error = Simple<char>> {
  parse_instruction()
    .padded()
    .then_ignore(comment().or_not())
    .separated_by(newline().or_not())
    .allow_trailing()
    .then_ignore(end())
    .map(|instructions| Program { instructions })
}

fn parse_instruction() -> impl Parser<char, Instruction, Error = Simple<char>> {
  choice((
    parse_br(),
    parse_add(),
    parse_ld(),
    parse_st(),
    parse_jsr(),
    parse_and(),
    parse_ldr(),
    parse_str(),
    parse_rti(),
    parse_not(),
    parse_ldi(),
    parse_sti(),
    parse_jmp(),
    parse_res(),
    parse_lea(),
    parse_trap(),
    parse_halt(),
  ))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::instructions::TrapVect;
  use crate::registers::Register;

  #[test]
  fn test_parse_instruction() {
    assert_eq!(
      parse_instruction().parse("add r0, r1, r2").unwrap(),
      Instruction::Add1(Register::R0, Register::R1, Register::R2)
    );
    assert_eq!(
      parse_instruction().parse("lea r0, x2").unwrap(),
      Instruction::Lea(Register::R0, 0x2)
    );
    assert_eq!(
      parse_instruction().parse("trap tgetc").unwrap(),
      Instruction::Trap(TrapVect::GetC)
    );
  }

  #[test]
  fn test_parse() {
    assert_eq!(
      parse_program().parse(
        "add r0, r1, r2
        lea r0, x2
        trap tgetc
        "
      ),
      Ok(Program {
        instructions: vec![
          Instruction::Add1(Register::R0, Register::R1, Register::R2),
          Instruction::Lea(Register::R0, 0x2),
          Instruction::Trap(TrapVect::GetC),
        ]
      })
    );
  }

  #[test]
  fn test_parse_with_comments() {
    assert_eq!(
      parse_program().parse(
        "add r0, r1, r2 ; comment
        lea r0, x2
        trap tgetc ; awa
        "
      ),
      Ok(Program {
        instructions: vec![
          Instruction::Add1(Register::R0, Register::R1, Register::R2),
          Instruction::Lea(Register::R0, 0x2),
          Instruction::Trap(TrapVect::GetC),
        ]
      })
    );
  }
}
