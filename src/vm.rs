use crate::chunk::Chunk;
use crate::opcode::OpCode;
use crate::value::Value;
use std::fmt;

#[derive(Debug)]
pub enum VMError {
    Compile,
    Runtime,
}

type VMStack = Vec<Value>;

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: VMStack,
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "An error occured {:?}", self)
    }
}

impl VM {
    pub fn new() -> Self {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: VMStack::new(),
        }
    }

    pub fn interpret(&mut self, source: &str) -> Result<(), VMError> {
        self.ip = 0; // ?
        self.run()
    }

    fn run(&mut self) -> Result<(), VMError> {
        loop {
            if self.ip >= self.chunk.len() {
                return Ok(());
            }

            #[cfg(debug_assertions)]
            {
                print!("          ");
                for val in &self.stack {
                    print!("[{}]", val);
                }
                println!("");
                self.chunk.dissassemble_instruction(self.ip);
            }

            match self.chunk.next_byte(&mut self.ip).into() {
                OpCode::Return => {
                    println!("{}", self.pop());
                    return Ok(());
                }
                OpCode::Constant => {
                    let val = self.chunk.get_constant(&mut self.ip, false);
                    self.push(val);
                }
                OpCode::ConstantLong => {
                    let val = self.chunk.get_constant(&mut self.ip, true);
                    self.push(val);
                }
                OpCode::Negate => {
                    let val = self.pop();
                    self.push(-val);
                }
                OpCode::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a + b);
                }
                OpCode::Subtract => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a - b);
                }
                OpCode::Multiply => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a * b);
                }
                OpCode::Divide => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a / b);
                }
                _ => return Err(VMError::Compile),
            }
        }
    }

    fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
}
