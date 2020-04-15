use std::fmt;

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    Unknown = 0,
    Return = 1,
}

impl From<u8> for OpCode {
    fn from(orig: u8) -> Self {
        match orig {
            0x1 => Self::Return,
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
}

impl Chunk {
    pub fn new() -> Self {
        Chunk { code: Vec::new() }
    }

    pub fn append_op(&mut self, op: OpCode) {
        self.code.push(op as u8);
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

        let op: OpCode = self.code[offset].into();
        match op {
            OpCode::Unknown => {
                println!("Unknown opcode {}", self.code[offset]);
                offset + 1
            }
            OpCode::Return => Self::simple_instruction(op, offset),
        }
    }

    fn simple_instruction(instr: OpCode, offset: usize) -> usize {
        println!("{}", instr);
        offset + 1
    }
}
