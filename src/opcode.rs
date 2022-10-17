// >	Increment the data pointer (to point to the next cell to the right).
// <	Decrement the data pointer (to point to the next cell to the left).
// +	Increment (increase by one) the byte at the data pointer.
// -	Decrement (decrease by one) the byte at the data pointer.
// .	Output the byte at the data pointer.
// ,	Accept one byte of input, storing its value in the byte at the data pointer.
// [	If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
// ]	If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.

#[derive(Debug, PartialEq)]
pub enum Opcode {
    SHR = 0x3E,     // >
    SHL = 0x3C,     // <
    ADD = 0x2B,     // +
    SUB = 0x2D,     // -
    PUTCHAR = 0x2E, // .
    GETCAHR = 0x2C, // ,
    LB = 0x5B,      // [
    RB = 0x5D,      // ]
}

impl From<u8> for Opcode {
    fn from(u: u8) -> Self {
        match u {
            0x3E => Opcode::SHR,
            0x3C => Opcode::SHL,
            0x2B => Opcode::ADD,
            0x2D => Opcode::SUB,
            0x2E => Opcode::PUTCHAR,
            0x2C => Opcode::GETCAHR,
            0x5B => Opcode::LB,
            0x5D => Opcode::RB,
            _ => unreachable!(), // 没有匹配上述内容，说明 brainfuck 代码存在问题，直接退出程序
        }
    }
}

pub struct Code {
    pub instrs: Vec<Opcode>,
    pub jtable: std::collections::HashMap<usize, usize>,
}

impl Code {
    pub fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let dict: Vec<u8> = vec![
            Opcode::SHR as u8,
            Opcode::SHL as u8,
            Opcode::ADD as u8,
            Opcode::SUB as u8,
            Opcode::PUTCHAR as u8,
            Opcode::GETCAHR as u8,
            Opcode::LB as u8,
            Opcode::RB as u8,
        ];

        let instrs: Vec<Opcode> = data
            .iter()
            .filter(|x| dict.contains(x))
            .map(|x| Opcode::from(*x))
            .collect();

        let mut jstack: Vec<usize> = Vec::new();
        let mut jtable: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for (i, e) in instrs.iter().enumerate() {
            if Opcode::LB == *e {
                jstack.push(i);
            }
            if Opcode::RB == *e {
                let j = jstack.pop().ok_or("pop from empty list")?;
                jtable.insert(j, i);
                jtable.insert(i, j);
            }
        }
        Ok(Code { instrs, jtable })
    }
}
