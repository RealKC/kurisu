mod chunk;

use chunk::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::new();
    chunk.append_op(OpCode::Return);
    chunk.disassemble("test chunk")
}
