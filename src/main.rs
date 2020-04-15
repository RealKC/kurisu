mod chunk;
mod opcode;
mod value;

use chunk::Chunk;
use opcode::OpCode;

fn main() {
    let mut chunk = Chunk::new();
    let idx = chunk.add_constant(1.5);
    chunk.append(OpCode::Constant as u8, 0);
    chunk.append(idx, 0);

    chunk.append(OpCode::Return as u8, 123);

    chunk.disassemble("test chunk")
}
