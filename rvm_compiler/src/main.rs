use std::env::args;
use std::fs::{read_to_string, File};
use std::io::Write;

use rvm_compiler::parsing::print_errors;

fn main() {
  let (in_file, out_file) = (args().nth(1), args().nth(2));
  if in_file.is_none() || out_file.is_none() {
    println!("Usage: rvm_compiler <in_file> <out_file>");
    return;
  }

  let in_file = in_file.unwrap();
  let out_file = out_file.unwrap();

  let contents = read_to_string(in_file).unwrap().to_ascii_lowercase();

  let program = match rvm_compiler::parsing::parse(&contents) {
    Ok(program) => program,
    Err(errs) => {
      print_errors(&contents, errs);

      panic!("Failed to parse program");
    }
  };

  let bytecode = rvm_compiler::serialize(&program);

  let mut file = File::create(&out_file).unwrap();

  file.write_all(&bytecode).unwrap();

  println!("written to `{out_file}`");
}
