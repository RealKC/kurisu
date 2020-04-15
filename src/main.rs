mod chunk;
mod value;

use chunk::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::new();
    let idx = chunk.add_constant(1.5);
    chunk.append(OpCode::Constant as u8);
    chunk.append(idx);
    chunk.disassemble("test chunk")
}
