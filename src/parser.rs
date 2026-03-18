/// opcodeをバイナリに変換する
pub fn parse_mnemonic(line: String) -> u8 {
    let splited: Vec<String> = line
        .split(' ')
        .to_owned()
        .map(|x| x.to_lowercase())
        .collect();
    let mut order = String::new();
    if splited.len() == 1 {
        order = String::from("11010000");
    } else {
        let register_operand: Vec<&str> = splited[1].split(',').collect();
        let mut register: &str = "";
        let mut operand: &str = "";
        if register_operand.len() == 2 {
            register = register_operand[0];
            operand = register_operand[1];
        }
        order = match splited[0].as_str() {
            "add" => match register {
                "a" => String::from("0000") + operand,
                "b" => String::from("0101") + operand,
                _ => unreachable!("ADD mnimonic is only use register `A` or `B`"),
            },
            "mov" => {
                if register == "a" {
                    if operand == "b" {
                        String::from("0001") + operand
                    } else {
                        String::from("0011") + operand
                    }
                } else if register == "b" {
                    if operand == "a" {
                        String::from("0100") + operand
                    } else {
                        String::from("0111") + operand
                    }
                } else {
                    unreachable!();
                }
            }
            "jmp" => String::from("1111") + splited[1].as_str(),
            "jnc" => String::from("1110") + splited[1].as_str(),
            "in" => {
                if register == "a" {
                    String::from("0010") + operand
                } else if splited[1] == "b" {
                    String::from("0110") + operand
                } else {
                    unreachable!()
                }
            }
            "out" => {
                if register == "b" {
                    String::from("10010000")
                } else {
                    String::from("1011") + splited[1].as_str()
                }
            }
            _ => {
                unreachable!("Not defined nimonic!");
            }
        };
    }
    let bin = u8::from_str_radix(order.as_str(), 2).unwrap();
    bin
}
