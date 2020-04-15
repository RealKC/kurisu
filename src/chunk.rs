use crate::value::Value;
use std::fmt;
#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    Unknown = 0,
    Return = 1,
    Constant,
}

impl From<u8> for OpCode {
    fn from(orig: u8) -> Self {
        match orig {
            0x1 => Self::Return,
            0x2 => Self::Constant,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<u32>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn append(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, val: Value) -> u8 {
        self.constants.push(val);
        (self.constants.len() - 1) as u8
    }

    pub fn disassemble(&self, name: &str) {
        println!("-== {} ==-", name);

        let mut offset = 0usize;
        while offset < self.code.len() {
            offset = self.dissassemble_instruction(offset);
        }
    }

    fn dissassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ")
        } else {
            print!("{:04} ", self.lines[offset])
        }

        let op: OpCode = self.code[offset].into();
        match op {
            OpCode::Unknown => {
                println!("Unknown opcode {}", self.code[offset]);
                offset + 1
            }
            OpCode::Return => Self::simple_instruction(op, offset),
            OpCode::Constant => self.constant_instruction(op, offset),
        }
    }

    fn simple_instruction(instr: OpCode, offset: usize) -> usize {
        println!("{}", instr);
        offset + 1
    }

    fn constant_instruction(&self, instr: OpCode, offset: usize) -> usize {
        let constant_idx = self.code[offset + 1];
        println!(
            "{} {} '{}'",
            instr, constant_idx, self.constants[constant_idx as usize]
        );
        offset + 2
    }
}
