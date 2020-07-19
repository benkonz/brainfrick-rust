use std::collections::{HashMap, VecDeque};
use std::io::prelude::*;
use std::io::stdin;

const MEMORY_CAPACITY: usize = 30_000;

pub fn execute(program: &[u8]) -> Result<(), String> {
    let bracket_map = create_bracket_map(program)?;

    let mut memory = [0u8; MEMORY_CAPACITY];
    let mut ptr = 0;
    let mut pc = 0;
    let mut stdin_buffer = [0];

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
                memory[ptr] = memory[ptr].wrapping_add(1);
                pc += 1;
            }
            '-' => {
                memory[ptr] = memory[ptr].wrapping_sub(1);
                pc += 1;
            }
            '.' => {
                print!("{}", memory[ptr] as char);
                pc += 1;
            }
            ',' => {
                stdin()
                    .read_exact(&mut stdin_buffer)
                    .map_err(|e| format!("{:?}", e))?;
                memory[ptr] = stdin_buffer[0];
                pc += 1;
            }
            '[' => {
                if memory[ptr] == 0 {
                    pc = bracket_map[&pc];
                }
                pc += 1;
            }
            ']' => {
                if memory[ptr] != 0 {
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

fn create_bracket_map(program: &[u8]) -> Result<HashMap<usize, usize>, String> {
    let mut brackets = VecDeque::new();
    let mut bracket_map = HashMap::new();
    for (i, c) in program.iter().enumerate() {
        match *c as char {
            '[' => brackets.push_front(i),
            ']' => {
                if let Some(j) = brackets.pop_front() {
                    bracket_map.insert(i, j);
                    bracket_map.insert(j, i);
                } else {
                    return Err(format!(
                        "syntax error: no matching `[` for `]` at index: {}",
                        i
                    ));
                }
            }
            _ => (),
        }
    }

    if let Some(i) = brackets.get(0) {
        Err(format!(
            "syntax error: no matching `]` for `[` at index: {}",
            i
        ))
    } else {
        Ok(bracket_map)
    }
}
