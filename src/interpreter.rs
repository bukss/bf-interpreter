use std::collections::HashMap;

fn throw (error: String) {
    println!("{}", error);
    std::process::exit(0);
}

pub enum Instruction {
    Increment,
    Decrement,
    MoveLeft,
    MoveRight,
    StartLoop(u32), // u32 is the index of the end of the loop
    EndLoop(u32),   // u32 is the index of the start of the loop
    Output,
    Input,
    Halt,
}

pub struct Interpreter {
    pub instructions: Vec<Instruction>,
    pub pointer: usize,
    pub code: String,
    pub loops: HashMap<u32, u32>,
    pub input: String,
    pub tape: [u8; 3000]
}

impl Interpreter {
    fn build_loop_map (&mut self) {
        /* Creates mapping of loops and their endpoints for easy jumping around the code at loops
         * endpoints and startpoints */

        let mut open_loops: Vec<u32> = Vec::new();
        for (i, c) in self.code.chars().enumerate() {
            let i: u32 = i as u32;
            if c == '[' {
                open_loops.push(i)
            } else if c == ']' {
                let start = match open_loops.pop() {
                    Some(n) => n,
                    None    => {
                        throw(format!("Loop Error: Closure of nonexistent loop\nChar: ']' Pos: {}", i));
                        0
                    }
                };
                self.loops.insert(start, i);
                self.loops.insert(i, start);
            }
        }
        if open_loops.len() > 0 {
            throw(format!("Loop Error: At least one unclosed loop\nChar: '[', Pos: {}", open_loops[0]));
        }
    }
    
    pub fn build_instructions (&mut self) { 
        /* Converts the code string into a vector of instructions  */
        
        self.build_loop_map();
        for (i, c) in self.code.chars().enumerate() {
            let i: u32 = i as u32;
            if c == '+' {
                self.instructions.push(Instruction::Increment);
            
            } else if c == '-' {
                self.instructions.push(Instruction::Decrement);
            
            } else if c == '<' {
                self.instructions.push(Instruction::MoveLeft);

            } else if c == '>' {
                self.instructions.push(Instruction::MoveRight);
            
            } else if c == '[' {
                let end = self.loops.get(&i).unwrap();
                self.instructions.push(Instruction::StartLoop(*end));

            } else if c == ']' {
                let start = self.loops.get(&i).unwrap();
                self.instructions.push(Instruction::EndLoop(*start));
            
            } else if c == '.' {
                self.instructions.push(Instruction::Output);
            
            } else if c == ',' {
                self.instructions.push(Instruction::Input);
            }
        }
        self.instructions.push(Instruction::Halt);
    }

    pub fn run(&mut self) {
        /* Loops over the vec of instructions and executes the corresponding code */
        let tape_size: usize = self.tape.len() - 1; // Represents the last index of the tape rather than the len
        let mut input_pointer: usize = 0;
        let inputs: Vec<u8> = self.input.clone().into_bytes();
        let mut instruction_pointer: usize = 0;
        loop {
            let instruction = &self.instructions[instruction_pointer]; 
            let cell = &mut self.tape[self.pointer]; 
            match *instruction {
                Instruction::Increment => {
                    *cell = cell.overflowing_add(1).0; 
                },
                Instruction::Decrement => {
                    *cell = cell.overflowing_sub(1).0;
                },
                Instruction::MoveRight => {
                    self.pointer = self.pointer.overflowing_sub(1).0;
                    if self.pointer > tape_size {
                        self.pointer = tape_size;
                    }
                }
                Instruction::MoveLeft => {
                    self.pointer += 1;
                    if self.pointer > tape_size {
                        self.pointer = 0;
                    }
                },
                Instruction::StartLoop(n) => {
                    if *cell == 0 {
                        instruction_pointer = n as usize;
                    }   
                },
                Instruction::EndLoop(n) => {
                    if *cell != 0 {
                        instruction_pointer = n as usize;
                    }
                },
                Instruction::Output => {
                    print!("{}", *cell as char);
                },
                Instruction::Input => {
                    let c = match inputs.get(input_pointer) {
                        Some(a) => *a,
                        None    => 0
                    };
                    *cell = c;
                    input_pointer += 1;
                },
                Instruction::Halt => {
                    break;
                }
            }
        instruction_pointer += 1;
        }
    }
}
