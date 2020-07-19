use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, stdin};
use std::collections::{HashMap, VecDeque};

const ARRAY_CAPACITY: usize = 30_000;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("requires a brainfuck file!");
        return Ok(());
    }

    let mut f = File::open(&args[1])?;
    let mut program = Vec::new();
    f.read_to_end(&mut program)?;

    let mut bracket_map: HashMap<usize, usize> = HashMap::new();
    let mut brackets: VecDeque<usize> = VecDeque::new();
    for (i, c) in program.iter().enumerate() {
        match *c as char {
            '[' => brackets.push_front(i),
            ']' => {
                if let Some(j) = brackets.pop_front() {
                    bracket_map.insert(i, j);
                    bracket_map.insert(j, i);
                } else {
                    println!("syntax error: no matching `[` for `]` at index: {}", i);
                    return Ok(());
                }
            },
            _ => ()
        }
    }

    if let Some(i) = brackets.get(0) {
        println!("syntax error: no matching `]` for `[` at index: {}", i);
        return Ok(());
    }

    let mut stdin_buffer = [0];

    let mut array = [0u8; ARRAY_CAPACITY];
    let mut ptr = 0;
    let mut pc = 0;

    while pc < program.len() {
        match program[pc] as char {
            '>' => {
                ptr += 1;
                pc += 1;
            }
            '<' => {
                ptr -= 1;
                pc += 1;
            }
            '+' => {
                array[ptr] = array[ptr].wrapping_add(1);
                pc += 1;
            }
            '-' => {
                array[ptr] = array[ptr].wrapping_sub(1);
                pc += 1;
            }
            '.' => {
                print!("{}", array[ptr] as char);
                pc += 1;
            }
            ',' => {
                stdin().read_exact(&mut stdin_buffer)?;
                array[ptr] = stdin_buffer[0];
                pc += 1;
            }
            '[' => {
                if array[ptr] == 0 {
                    pc = bracket_map[&pc];
                }
                pc += 1;

            }
            ']' => {
                if array[ptr] != 0 {
                    pc = bracket_map[&pc];
                }
                pc += 1;
            }
            _ => {
                pc += 1;
            }
        }
    }

    Ok(())
}
