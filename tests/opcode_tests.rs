use td4_emulator::opcode::OpCode;

#[test]
fn opcode_from_u8_basic() {
    assert_eq!(OpCode::from(0b0000), OpCode::AddA);
    assert_eq!(OpCode::from(0b0101), OpCode::AddB);
    assert_eq!(OpCode::from(0b0011), OpCode::MovA);
    assert_eq!(OpCode::from(0b0111), OpCode::MovB);
    assert_eq!(OpCode::from(0b0001), OpCode::MovA2B);
    assert_eq!(OpCode::from(0b0100), OpCode::MovB2A);
    assert_eq!(OpCode::from(0b1111), OpCode::Jmp);
    assert_eq!(OpCode::from(0b1110), OpCode::Jnc);
    assert_eq!(OpCode::from(0b0010), OpCode::InA);
    assert_eq!(OpCode::from(0b0110), OpCode::InB);
    assert_eq!(OpCode::from(0b1001), OpCode::OutB);
    assert_eq!(OpCode::from(0b1011), OpCode::Out);
    assert_eq!(OpCode::from(0b1101), OpCode::Brk);
}
