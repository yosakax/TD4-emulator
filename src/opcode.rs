#[derive(Debug)]
pub enum OpCode {
    // 命令を書く
    AddA = 0b0000,   // register A + operand
    AddB = 0b0101,   // register B + operand
    MovA = 0b0011,   // register A <- operand
    MovB = 0b0111,   // register B <- operand
    MovA2B = 0b0001, // register A <- register B
    MovB2A = 0b0100, // register B <- register A
    Jmp = 0b1111,    // pc <- operand
    Jnc = 0b1110,    // if !carry {pc <- operand}
    InA = 0b0010,    // register A <- port.input
    InB = 0b0110,    // register B <- port.input
    OutB = 0b1001,   // port.output <- register B
    Out = 0b1011,    // port.output <- operand
    Brk = 0b1101,    // ** exit code.
}

impl From<u8> for OpCode {
    fn from(raw_opcode: u8) -> OpCode {
        match raw_opcode {
            0b0000 => OpCode::AddA,
            0b0101 => OpCode::AddB,
            0b0011 => OpCode::MovA,
            0b0111 => OpCode::MovB,
            0b0001 => OpCode::MovA2B,
            0b0100 => OpCode::MovB2A,
            0b1111 => OpCode::Jmp,
            0b1110 => OpCode::Jnc,
            0b0010 => OpCode::InA,
            0b0110 => OpCode::InB,
            0b1001 => OpCode::OutB,
            0b1011 => OpCode::Out,
            0b1101 => OpCode::Brk,
            _ => unreachable!(),
        }
    }
}
