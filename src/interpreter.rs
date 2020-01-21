use std::collections::HashMap;

#[derive(Debug)]
pub enum Instruction {
    Increment,
    Decrement,
    MoveLeft,
    MoveRight,
    StartLoop(usize), // usize is the index of the end of the loop
    EndLoop(usize),   // usize is the index of the start of the loop
    Output,
    Input,
    Halt,
}

#[derive(Debug, thiserror::Error)]
pub enum BuildLoopMapError {
    #[error("found loops with missing close instructions at indices {0:?}")]
    StrayOpens(Vec<usize>),
    #[error("found loop with missing open instructions at index {0}")]
    StrayClose(usize),
}

// Creates mapping of loops and their endpoints for easy jumping around the code at loops endpoints and startpoints
fn build_loop_map(code: &str) -> Result<HashMap<usize, usize>, BuildLoopMapError> {
    use BuildLoopMapError::*;

    let code = code
        .chars()
        .filter(|c| ['+', '-', '<', '>', '[', ']', '.', ','].contains(&c));
    let mut loops = HashMap::new();
    let mut open_loops = Vec::new();

    for (i, c) in code.enumerate() {
        match c {
            '[' => open_loops.push(i),
            ']' => {
                let start = open_loops.pop().ok_or_else(|| StrayClose(i))?;

                loops.insert(start, i);
                loops.insert(i, start);
            }
            _ => {}
        }
    }

    if !open_loops.is_empty() {
        return Err(StrayOpens(open_loops));
    }

    Ok(loops)
}

// Converts the code string into a vector of instructions
pub fn build_instructions(code: &str, loops: &HashMap<usize, usize>) -> Vec<Instruction> {
    use Instruction::*;

    let mut instructions = Vec::new();
    let code = code
        .chars()
        .filter(|c| ['+', '-', '<', '>', '[', ']', '.', ','].contains(&c));

    for (i, c) in code.enumerate() {
        let instruction = match c {
            '+' => Increment,
            '-' => Decrement,
            '<' => MoveLeft,
            '>' => MoveRight,
            '[' => StartLoop(*loops.get(&i).unwrap()),
            ']' => EndLoop(*loops.get(&i).unwrap()),
            '.' => Output,
            ',' => Input,
            _ => continue,
        };

        instructions.push(instruction);
    }

    instructions.push(Instruction::Halt);

    instructions
}

#[derive(Debug, thiserror::Error)]
pub enum NewInterpreterError {
    #[error("error while building loop map")]
    BuildLoopMap(#[from] BuildLoopMapError),
}

pub struct Interpreter {
    pub instructions: Vec<Instruction>,
    pub instruction_pointer: usize,
    pub data_pointer: usize,
    pub tape: [u8; 30000],
}

impl Interpreter {
    pub fn new(code: &str) -> Result<Self, NewInterpreterError> {
        let loops = build_loop_map(code)?;
        let instructions = build_instructions(code, &loops);

        Ok(Self {
            instructions,
            instruction_pointer: 0,
            data_pointer: 0,
            tape: [0; 30000],
        })
    }

    // Loops over the vec of instructions and executes the corresponding code
    pub fn run(&mut self, mut input: impl Iterator<Item = u8>) {
        while self.step(&mut input) {}
    }

    pub fn step(&mut self, mut input: impl Iterator<Item = u8>) -> bool {
        use Instruction::*;

        let cell = &mut self.tape[self.data_pointer];
        match self.instructions[self.instruction_pointer] {
            Increment => *cell = cell.wrapping_add(1),
            Decrement => *cell = cell.wrapping_sub(1),
            MoveRight if self.data_pointer == 2999 => self.data_pointer = 0,
            MoveRight => self.data_pointer += 1,
            MoveLeft if self.data_pointer == 0 => self.data_pointer = 2999,
            MoveLeft => self.data_pointer -= 1,
            StartLoop(n) if *cell == 0 => self.instruction_pointer = n,
            EndLoop(n) if *cell != 0 => self.instruction_pointer = n,
            Output => print!("{}", *cell as char),
            Input => *cell = input.next().unwrap(),
            Halt => return false,
            _ => {}
        }

        self.instruction_pointer += 1;

        true
    }
}
