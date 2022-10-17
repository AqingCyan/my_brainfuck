mod opcode;

use opcode::Code;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let data = std::fs::read(&args[1])?; // 取命令行参数
    let code = Code::from(data)?;

    println!("{:?}", code.instrs);

    Ok(())
}
