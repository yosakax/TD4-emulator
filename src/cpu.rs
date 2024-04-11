use std::process::exit;

use crate::{opcode::OpCode, port::Port, register::Register, rom::Rom};

pub struct Cpu {
    pub pc: u8,
    pub carry: bool,
    pub rom: Rom,
    register: Register,
    pub port: Port,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: 0,
            carry: false,
            rom: Rom::new(),
            register: Register::new(),
            port: Port::new(),
        }
    }

    pub fn load_file(&mut self, file_path: &str) {
        self.rom.load_bin(file_path);
    }
    pub fn load_file_mnimonic(&mut self, file_path: &str) {
        self.rom.load_mnemonic(file_path);
    }

    fn fetch(&mut self) -> (OpCode, u8) {
        let memory = self.rom.memory[self.pc as usize];
        let opcode = OpCode::from(memory >> 4);
        let operand = memory & (0b00001111);
        (opcode, operand)
    }

    pub fn execute(&mut self) {
        let (opcode, operand) = self.fetch();
        // eprintln!("{:?} {:04b}", opcode, operand);
        match opcode {
            OpCode::AddA => {
                self.register.a += operand;
                self.carry = self.register.a >> 4 & 1 == 1;
                self.register.a &= 0b1111;
            }
            OpCode::AddB => {
                self.register.b += operand;
                self.carry = self.register.b >> 4 & 1 == 1;
                self.register.b &= 0b1111;
            }
            OpCode::MovA => {
                self.register.a = operand;
                self.carry = false;
            }
            OpCode::MovB => {
                self.register.b = operand;
                self.carry = false;
            }
            OpCode::MovA2B => {
                self.register.a = self.register.b;
                self.carry = false;
            }
            OpCode::MovB2A => {
                self.register.b = self.register.a;
                self.carry = false;
            }
            OpCode::InA => {
                self.register.a = self.port.input;
                self.carry = false;
            }
            OpCode::InB => {
                self.register.b = self.port.input;
                self.carry = false;
            }
            OpCode::Out => {
                self.port.output = operand;
                self.carry = false;
            }
            OpCode::OutB => {
                self.port.output = self.register.b;
                self.carry = false;
            }
            OpCode::Jmp => {
                self.pc = operand;
                self.carry = false;
                return;
            }
            OpCode::Jnc => {
                if !self.carry {
                    self.pc = operand;
                    return;
                }
                self.carry = false;
            }
            OpCode::Brk => {
                exit(0);
            }
        }
        self.pc = (self.pc + 1) & 0b1111;
    }
}
