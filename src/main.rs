use std::{collections::HashMap,
    io::stdin};

mod interpreter;
use interpreter::Interpreter;

fn main() {
    let mut code = String::new();
    println!("Brainf code:");
    stdin().read_line(&mut code).unwrap();

    let mut input = String::new();
    println!("Inputs for the program:");
    stdin().read_line(&mut input).unwrap();

    let mut program = Interpreter {
        instructions: Vec::new(),
        pointer: 0,
        code,
        loops: HashMap::new(),
        input: input.trim().to_string(),
        tape: [0; 3000]
    };

    program.build_instructions();
    program.run();
}
