use crate::opcode::OpCode;
use crate::value::Value;

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

    pub fn append_constant(&mut self, val: Value, line: u32) {
        self.constants.push(val);
        let len = (self.constants.len() - 1) as u32;
        if len < 256 {
            self.append(OpCode::Constant as u8, line);
            self.append(len as u8, line);
        } else {
            // In this case we emit OpCode::ConstantLong which has a 4 byte operand

            self.append(OpCode::ConstantLong as u8, line);
            self.append(((len & 0xff_00_00_00) >> 24) as u8, line);
            self.append(((len & 0x00_ff_00_00) >> 16) as u8, line);
            self.append(((len & 0x00_00_ff_00) >> 8) as u8, line);
            self.append((len & 0x00_00_00_ff) as u8, line);
        }
    }

    pub fn next_byte(&self, ip: &mut usize) -> u8 {
        let byte = self.code[*ip];
        *ip += 1;
        byte
    }

    pub fn get_constant(&self, ip: &mut usize, is_long: bool) -> Value {
        let idx = if is_long {
            let long_index = (self.code[*ip] as usize) << 24
                | (self.code[*ip + 1] as usize) << 16
                | (self.code[*ip + 2] as usize) << 8
                | (self.code[*ip + 3] as usize);
            *ip += 4;
            long_index
        } else {
            let short_index = self.code[*ip] as usize;
            *ip += 1;
            short_index
        };
        self.constants[idx]
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn disassemble(&self, name: &str) {
        println!("-== {} ==-", name);

        let mut offset = 0usize;
        while offset < self.code.len() {
            offset = self.dissassemble_instruction(offset);
        }
    }

    pub fn dissassemble_instruction(&self, offset: usize) -> usize {
        print!("o:{:04} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("     | ")
        } else {
            print!("l:{:04} ", self.lines[offset])
        }

        let op: OpCode = self.code[offset].into();
        match op {
            OpCode::Unknown => {
                println!("Unknown opcode ({})", self.code[offset]);
                offset + 1
            }
            OpCode::Return => Self::simple_instruction(op, offset),
            OpCode::Constant => self.constant_instruction(op, offset, false),
            OpCode::ConstantLong => self.constant_instruction(op, offset, true),
            OpCode::Negate => Self::simple_instruction(op, offset),
            OpCode::Add => Self::simple_instruction(op, offset),
            OpCode::Subtract => Self::simple_instruction(op, offset),
            OpCode::Multiply => Self::simple_instruction(op, offset),
            OpCode::Divide => Self::simple_instruction(op, offset),
        }
    }

    fn simple_instruction(instr: OpCode, offset: usize) -> usize {
        println!("{}", instr);
        offset + 1
    }

    fn constant_instruction(&self, instr: OpCode, offset: usize, is_long: bool) -> usize {
        if is_long {
            let constant_idx = (self.code[offset + 1] as u32) << 24
                | (self.code[offset + 2] as u32) << 16
                | (self.code[offset + 3] as u32) << 8
                | (self.code[offset + 4] as u32);
            println!(
                "{} {} '{}'",
                instr, constant_idx, self.constants[constant_idx as usize]
            );
            offset + 5
        } else {
            let constant_idx = self.code[offset + 1];
            println!(
                "{} {} '{}'",
                instr, constant_idx, self.constants[constant_idx as usize]
            );
            offset + 2
        }
    }
}
