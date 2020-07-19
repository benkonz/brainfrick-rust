mod runtime;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("requires a brainfuck file!");
        return Ok(());
    }

    let mut f = File::open(&args[1]).map_err(|e| format!("{:?}", e))?;
    let mut program = Vec::new();
    f.read_to_end(&mut program)
        .map_err(|e| format!("{:?}", e))?;

    runtime::execute(&program)
}
