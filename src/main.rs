#![allow(unused)]
mod brainfuck;
use std::str::Chars;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut args = args.iter();
    args.next();
    match args.next() {
        Some(path) => match std::fs::read_to_string(path) {
            Ok(text) => {
                let instrs = brainfuck::Instr::generate(text.as_str());
                let mut program = brainfuck::Program::new();
                program.instrs(&instrs);
            }
            Err(e) => println!("{e}")
        }
        None => {}
    }
}
