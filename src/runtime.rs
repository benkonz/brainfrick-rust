use std::collections::{HashMap, VecDeque};
use std::io::prelude::*;
use std::io::stdin;

const MEMORY_CAPACITY: usize = 30_000;

pub struct Runtime {
    memory: [u8; MEMORY_CAPACITY],
    ptr: usize,
    pc: usize,
    bracket_map: HashMap<usize, usize>,
    brackets: VecDeque<usize>,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            memory: [0; MEMORY_CAPACITY],
            ptr: 0,
            pc: 0,
            bracket_map: HashMap::new(),
            brackets: VecDeque::new()
        }
    }

    pub fn execute(&mut self, program: &[u8]) -> Result<(), String> {
        self.create_bracket_map(program)?;

        let mut stdin_buffer = [0];

        while self.pc < program.len() {
            match program[self.pc] as char {
                '>' => {
                    self.ptr += 1;
                    self.pc += 1;
                }
                '<' => {
                    self.ptr -= 1;
                    self.pc += 1;
                }
                '+' => {
                    self.memory[self.ptr] = self.memory[self.ptr].wrapping_add(1);
                    self.pc += 1;
                }
                '-' => {
                    self.memory[self.ptr] = self.memory[self.ptr].wrapping_sub(1);
                    self.pc += 1;
                }
                '.' => {
                    print!("{}", self.memory[self.ptr] as char);
                    self.pc += 1;
                }
                ',' => {
                    stdin().read_exact(&mut stdin_buffer).map_err(|e| format!("{:?}", e))?;
                    self.memory[self.ptr] = stdin_buffer[0];
                    self.pc += 1;
                }
                '[' => {
                    if self.memory[self.ptr] == 0 {
                        self.pc = self.bracket_map[&self.pc];
                    }
                    self.pc += 1;
                }
                ']' => {
                    if self.memory[self.ptr] != 0 {
                        self.pc = self.bracket_map[&self.pc];
                    }
                    self.pc += 1;
                }
                _ => {
                    self.pc += 1;
                }
            }
        }

        Ok(())
    }

    fn create_bracket_map(&mut self, program: &[u8]) -> Result<(), String> {
        for (i, c) in program.iter().enumerate() {
            match *c as char {
                '[' => self.brackets.push_front(i),
                ']' => {
                    if let Some(j) = self.brackets.pop_front() {
                        self.bracket_map.insert(i, j);
                        self.bracket_map.insert(j, i);
                    } else {
                        return Err(format!("syntax error: no matching `[` for `]` at index: {}", i));
                    }
                }
                _ => (),
            }
        }

        if let Some(i) = self.brackets.get(0) {
            Err(format!("syntax error: no matching `]` for `[` at index: {}", i))
        } else {
            Ok(())
        }
    }
}
