use anyhow::Context;
use interpreter::Interpreter;
use std::io::stdin;

mod interpreter;

fn main() {
    if let Err(error) = run() {
        eprintln!("ran into an error: {:#}", error);
    }
}

fn run() -> anyhow::Result<()> {
    let mut code = String::new();
    println!("Brainf code:");
    stdin()
        .read_line(&mut code)
        .context("failed reading brainf code")?;

    let mut input = String::new();
    println!("Inputs for the program:");
    stdin()
        .read_line(&mut input)
        .context("failed reading program input")?;
    let mut input = input.as_bytes().iter().copied().cycle();

    let mut program = Interpreter::new(&code).context("failed building interpreter")?;

    program.run(&mut input);

    Ok(())
}
