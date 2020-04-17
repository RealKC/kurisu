mod chunk;
mod opcode;
mod value;
mod vm;

use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::process;
use vm::VMError;
use vm::VM;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_file(&args[1].as_ref());
    } else {
        println!("Usage: kurisu [path]");
        process::exit(1);
    }
}

fn repl() {
    let mut vm = VM::new();
    let stdin = io::stdin();
    loop {
        print!("> ");
        let mut line = String::new();
        stdin
            .lock()
            .read_line(&mut line)
            .expect("Could not read a line from stdin");
        match vm.interpret(line.as_ref()) {
            Ok(()) => (),
            Err(e) => {
                println!(
                    "Error in line \n\t{}\n{}",
                    line,
                    match e {
                        VMError::Runtime => "A runtime error occured",
                        VMError::Compile => "An error related to compiling your code occured",
                    }
                );
            }
        }
    }
}

fn run_file(file: &str) {
    fn interpret(file: &str) -> Result<(), VMError> {
        let mut vm = VM::new();
        let contents =
            fs::read_to_string(file).expect(format!("Could not open file {}\n", file).as_ref());
        vm.interpret(contents.as_ref())
    }
    match interpret(file) {
        Ok(()) => (),
        Err(e) => {
            println!("Error running your file {}", e);
            process::exit(3);
        }
    }
}
