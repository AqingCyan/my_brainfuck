mod opcode;

use std::io::{Read, Write};

use opcode::{Code, Opcode};

struct Interpreter {
    stack: Vec<u8>,
}

impl Interpreter {
    fn new() -> Self {
        Self { stack: vec![0; 1] }
    }

    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let code = Code::from(data)?;
        let code_len = code.instrs.len();
        let mut pc = 0; // Program counter 已执行代码指令数量
        let mut ps = 0; // Stack Pointer

        loop {
            // 执行结束 就 break
            if pc >= code_len {
                break;
            }
            match code.instrs[pc] {
                Opcode::SHR => {
                    ps += 1;
                    if ps == self.stack.len() {
                        self.stack.push(0);
                    }
                }
                Opcode::SHL => {
                    if ps != 0 {
                        ps -= 1;
                    }
                }
                opcode::Opcode::ADD => {
                    self.stack[ps] = self.stack[ps].overflowing_add(1).0;
                }
                opcode::Opcode::SUB => {
                    self.stack[ps] = self.stack[ps].overflowing_sub(1).0;
                }
                opcode::Opcode::PUTCHAR => {
                    std::io::stdout().write_all(&[self.stack[ps]])?;
                }
                opcode::Opcode::GETCAHR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[ps] = buf[0];
                }
                opcode::Opcode::LB => {
                    if self.stack[ps] == 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
                opcode::Opcode::RB => {
                    if self.stack[ps] != 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
            }

            pc += 1; // 执行一个指令就加一
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let data = std::fs::read(&args[1])?; // 取命令行参数

    let mut interpreter = Interpreter::new();
    interpreter.run(data);

    Ok(())
}
