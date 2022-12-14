use crate::*;

pub enum InstrResult {
    Instr(Instr),
    Char(Option<char>)
}
#[derive(Debug, Clone)]
pub enum Instr {
    Left, Right, Inc, Dec, Loop(Vec<Instr>),
    In, Out, OutNum
}
impl Instr {
    pub fn get(chars: &mut Chars) -> Result<InstrResult, ()> {
        match chars.next() {
            Some(c) => match c {
                '+' => Ok(InstrResult::Instr(Self::Inc)),
                '-' => Ok(InstrResult::Instr(Self::Dec)),
                '<' => Ok(InstrResult::Instr(Self::Left)),
                '>' => Ok(InstrResult::Instr(Self::Right)),
                ',' => Ok(InstrResult::Instr(Self::In)),
                '.' => Ok(InstrResult::Instr(Self::Out)),
                ':' => Ok(InstrResult::Instr(Self::OutNum)),
                '[' => {
                    let mut instrs: Vec<Instr> = vec![];
                    while let Ok(instr) = Self::get(chars) {
                        match instr {
                            InstrResult::Instr(instr) => instrs.push(instr),
                            InstrResult::Char(c) => if c == Some(']') { break }
                        }
                    }
                    Ok(InstrResult::Instr(Self::Loop(instrs)))
                }
                _ => Ok(InstrResult::Char(Some(c)))
            }
            None => Err(())
        }
    }
    pub fn generate(text: &str, ) -> Vec<Instr> {
        let mut instrs: Vec<Instr> = vec![];
        let mut chars = text.chars();
        while let Ok(instr) = Self::get(&mut chars) {
            match instr {
                InstrResult::Instr(instr) => instrs.push(instr),
                InstrResult::Char(_) => {}
            }
        }
        instrs
    }
}

pub struct Program {
    tape: [u8; 0xff],
    ptr: u8,
}
impl Program {
    pub fn new() -> Self { Self { tape: [0; 0xff], ptr: 0 } }
    pub fn interpret(&mut self, instr: &Instr) {
        match instr {
            Instr::Inc => self.tape[self.ptr as usize] = self.tape[self.ptr as usize].wrapping_add(1),
            Instr::Dec => self.tape[self.ptr as usize] = self.tape[self.ptr as usize].wrapping_sub(1),
            Instr::Right => self.ptr = self.ptr.wrapping_add(1),
            Instr::Left => self.ptr = self.ptr.wrapping_sub(1),
            Instr::In => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input);
                match input.trim().parse() {
                    Ok(n) => self.tape[self.ptr as usize] = n,
                    Err(_) => {}
                }
            }
            Instr::Out => print!("{}", self.tape[self.ptr as usize] as char),
            Instr::OutNum => print!("{}", self.tape[self.ptr as usize]),
            Instr::Loop(instr) => while self.tape[self.ptr as usize] != 0 { self.instrs(instr); }
        };
    }
    pub fn instrs(&mut self, instrs: &Vec<Instr>) { for instr in instrs { self.interpret(instr); } }
}