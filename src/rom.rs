use std::fs::File;
use std::io::*;

pub struct Rom {
    pub memory: Vec<u8>,
}

impl Rom {
    pub fn new() -> Self {
        Rom {
            memory: vec![0u8; 16],
        }
    }

    /// ファイルから読み込む
    pub fn load_bin(&mut self, file_path: &str) {
        let file = File::open(file_path).expect("Cannot read file.");
        let lines = BufReader::new(&file).lines();
        for (addr, line) in lines.enumerate() {
            if let Ok(bin_str) = line {
                let bin = u8::from_str_radix(&bin_str, 2).unwrap();
                // eprintln!("{:?}", self.memory);
                self.memory[addr] = bin;
            }
        }
    }
}
