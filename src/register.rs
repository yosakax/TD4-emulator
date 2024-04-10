pub struct Register {
    pub a: u8,
    pub b: u8,
}

impl Register {
    pub fn new() -> Self {
        Register { a: 0, b: 0 }
    }
}
