pub struct Port {
    pub input: u8,
    pub output: u8,
}

impl Port {
    pub fn new() -> Self {
        Port {
            input: 0,
            output: 0,
        }
    }
}
