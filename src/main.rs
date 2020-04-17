mod chunk;
mod opcode;
mod value;
mod vm;

use chunk::Chunk;
use opcode::OpCode;
use vm::VM;

fn main() {
    let mut chunk = Chunk::new();

    chunk.append_constant(1.0, 69);
    chunk.append(OpCode::Negate as u8, 666);
    chunk.append_constant(2.0, 69);
    chunk.append(OpCode::Add as u8, 666);
    chunk.append_constant(5., 5858);
    chunk.append(OpCode::Subtract as u8, 55);
    chunk.append_constant(3., 55);
    chunk.append(OpCode::Multiply as u8, 55);
    chunk.append_constant(6., 55);
    chunk.append(OpCode::Divide as u8, 55);

    chunk.append(OpCode::Return as u8, 420);

    chunk.disassemble("test chunk");

    let mut vm = VM::new();
    match vm.interpret(chunk) {
        Ok(()) => println!("All is well"),
        Err(e) => println!("May the Gods offer pity, because rustc won't: {}", e),
    }
}
