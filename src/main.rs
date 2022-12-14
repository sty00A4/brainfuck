#![allow(unused)]
mod brainfuck;
use std::str::Chars;

fn main() {
    let instrs = brainfuck::Instr::generate(",[>++<-]>.");
    let mut program = brainfuck::Program::new();
    program.instrs(&instrs);
}
