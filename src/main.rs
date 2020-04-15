mod chunk;
mod opcode;
mod value;

use chunk::Chunk;
use opcode::OpCode;

fn main() {
    let mut chunk = Chunk::new();

    for i in 0..257 {
        chunk.append_constant(1.5 + i as f64, i);
    }

    chunk.disassemble("test chunk")
}
