use crate::instructions::Instruction;

pub mod instructions;
pub mod parsing;
pub mod registers;

#[derive(Debug, PartialEq)]
pub struct Program {
  pub instructions: Vec<Instruction>,
}

pub fn serialize(program: &Program) -> Vec<u8> {
  let mut out: Vec<u16> = Vec::with_capacity(program.instructions.len() + 1);

  let header = 0b0011_0000_0000_0000;
  out.push(header);

  for instruction in &program.instructions {
    out.push(instruction.bytecode());
  }

  out.iter().flat_map(|&x| x.to_be_bytes()).collect()
}
