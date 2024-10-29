use std::io;

/// The `Interpreter` struct in Rust contains a memory array and a pointer for executing code.
/// 
/// Properties:
/// 
/// * `memory`: The `memory` property in the `Interpreter` struct is an array of unsigned 8-bit integers
/// with a length of 262,144 elements. This array is used to store data that the interpreter will work
/// with during its execution.
/// * `pointer`: The `pointer` property in the `Interpreter` struct is of type `usize`. It is used to
/// keep track of the current position in the memory array during interpretation of instructions.
pub(crate) struct Interpreter {
    memory: [u8; 262_144],
    pointer : usize
}

impl Interpreter {
    /// The function `new` initializes a new instance with a memory array of size 262,144 and a pointer
    /// set to 0 in Rust.
    /// 
    /// Returns:
    /// 
    /// A new instance of the struct with the `memory` array initialized to contain 262,144 elements,
    /// all set to 0, and the `pointer` field set to 0.
    pub fn new() -> Self {
        Self {
            memory: [0; 262_144],
            pointer: 0
        }
    }

    /// The function implements a Brainfuck interpreter in Rust, executing Brainfuck code provided as
    /// input.
    /// 
    /// Arguments:
    /// 
    /// * `src`: The `run` function you provided is an implementation of a Brainfuck interpreter in
    /// Rust. The `src` parameter is a string containing Brainfuck code that you want to execute. The
    /// function iterates over each character in the input code and performs the corresponding Brainfuck
    /// operation based on the character.
    pub fn run(&mut self, src: &str) {
        let mut i = 0;
        let char_vec: Vec<char> = src.chars().collect();

        while i < char_vec.len() {
            
            match char_vec[i] {
                '>' => self.pointer = (self.pointer + 1) % self.memory.len(),
                '<' => self.pointer = (self.pointer + self.memory.len() - 1) % self.memory.len(),
                '+' => self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1),
                '-' => self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1),
                '.' => print!("{}", self.memory[self.pointer] as char),
                ',' => {
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    self.memory[self.pointer] = input.trim().parse::<u8>().unwrap();
                }
                '[' => {
                    if self.memory[self.pointer] == 0 {
                        let mut bracket_count = 1;
                        let mut j = i + 1;
                        while bracket_count > 0 && j < char_vec.len() {
                            if char_vec[j] == '[' {
                                bracket_count += 1;
                            } else if char_vec[j] == ']' {
                                bracket_count -= 1;
                            }
                            j += 1;
                        }
                        i = j - 1;
                    }
                }
                ']' => {
                    if self.memory[self.pointer] != 0 {
                        let mut bracket_count = 1;
                        let mut j = i as u32 - 1;
                        while bracket_count > 0 && j >= 0 {
                            if char_vec[j as usize] == ']' {
                                bracket_count += 1;
                            } else if char_vec[j as usize] == '[' {
                                bracket_count -= 1;
                            }
                            j -= 1;
                        }
                        i = j as usize + 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }
    }
    
}