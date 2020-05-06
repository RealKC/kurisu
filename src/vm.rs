use crate::chunk::Chunk;
use crate::compiler;
use crate::object::Object;
use crate::opcode::OpCode;
use crate::value::Value;
use std::fmt;

const DEBUG_SHOW_DISASSEMBLY: bool = false;
const DEBUG_SHOW_STACK: bool = true;

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
        match compiler::compile(source) {
            Some(chunk) => {
                self.chunk = chunk;
                self.ip = 0;
                self.run()
            }
            None => Err(VMError::Compile),
        }
    }

    fn run(&mut self) -> Result<(), VMError> {
        loop {
            if self.ip >= self.chunk.len() {
                return Ok(());
            }

            if DEBUG_SHOW_STACK {
                for val in &self.stack {
                    print!("[{:?}]", val);
                }
                println!("");
            }
            if DEBUG_SHOW_DISASSEMBLY {
                print!("          ");
                self.chunk.dissassemble_instruction(self.ip);
            }

            match self.chunk.next_byte(&mut self.ip).into() {
                OpCode::Return => {
                    println!("{}", self.pop());
                    return Ok(());
                }
                OpCode::Constant => {
                    let val = self.chunk.get_constant(&mut self.ip, false).clone();
                    self.push(val);
                }
                OpCode::ConstantLong => {
                    let val = self.chunk.get_constant(&mut self.ip, true).clone();
                    self.push(val);
                }
                OpCode::Nil => self.push(Value::Nil),
                OpCode::True => self.push(Value::Boolean(true)),
                OpCode::False => self.push(Value::Boolean(false)),
                OpCode::Negate => {
                    if self.peek(0).is_number() {
                        let a = self.pop().to_number();
                        self.push(Value::Number(-a))
                    }
                }
                OpCode::Not => {
                    let val = self.pop().is_falsey();
                    self.push(Value::Boolean(val))
                }
                OpCode::Add => {
                    let b = self.peek(0);
                    let a = self.peek(1);

                    if a.is_string() && b.is_string() {
                        let b = self.pop();
                        let a = self.pop();
                        let new = format!("{}{}", a.as_string(), b.as_string());
                        self.push(Value::Obj(Box::new(Object::String(new))));
                    } else if a.is_number() && b.is_number() {
                        let b = self.pop().to_number();
                        let a = self.pop().to_number();
                        self.push(Value::Number(a + b));
                    } else {
                    }
                }
                OpCode::Subtract => match (self.peek(0), self.peek(1)) {
                    (Value::Number(_), Value::Number(_)) => {
                        let b = self.pop().to_number();
                        let a = self.pop().to_number();
                        self.push(Value::Number(a - b));
                    }
                    _ => {
                        self.runtime_error("Operands must be numbers");
                        return Err(VMError::Runtime);
                    }
                },
                OpCode::Multiply => match (self.peek(0), self.peek(1)) {
                    (Value::Number(_), Value::Number(_)) => {
                        let b = self.pop().to_number();
                        let a = self.pop().to_number();
                        self.push(Value::Number(a * b));
                    }
                    _ => {
                        self.runtime_error("Operands must be numbers");
                        return Err(VMError::Runtime);
                    }
                },
                OpCode::Divide => match (self.peek(0), self.peek(1)) {
                    (Value::Number(_), Value::Number(_)) => {
                        let b = self.pop().to_number();
                        let a = self.pop().to_number();
                        self.push(Value::Number(a / b));
                    }
                    _ => {
                        self.runtime_error("Operands must be numbers");
                        return Err(VMError::Runtime);
                    }
                },
                OpCode::Equal => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Boolean(a == b));
                }
                OpCode::Greater => match (self.peek(0), self.peek(1)) {
                    (Value::Number(_), Value::Number(_)) => {
                        let b = self.pop();
                        let a = self.pop();
                        self.push(Value::Boolean(a > b));
                    }
                    _ => {
                        self.runtime_error("Operands must be numbers");
                        return Err(VMError::Runtime);
                    }
                },
                OpCode::Less => match (self.peek(0), self.peek(1)) {
                    (Value::Number(_), Value::Number(_)) => {
                        let b = self.pop();
                        let a = self.pop();
                        self.push(Value::Boolean(a < b));
                    }
                    _ => {
                        self.runtime_error("Operands must be numbers");
                        return Err(VMError::Runtime);
                    }
                },
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

    fn peek(&self, distance: usize) -> &Value {
        &self.stack[self.stack.len() - 1 - distance]
    }

    fn runtime_error(&mut self, msg: &str) {
        eprintln!("{}", msg);

        let instruction = self.ip;
        let line = self.chunk.lines[instruction];
        eprintln!("[line {}] in script", line);
    }
}
