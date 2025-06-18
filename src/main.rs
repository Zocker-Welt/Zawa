mod tokenizer;
use crate::tokenizer::*;

mod expr;

mod parser;
use crate::parser::*;

mod interpreter;
use crate::interpreter::*;

use std::env;
use std::process::exit;
use std::fs;
use std::io::{self, BufRead, Write};

fn run(interpreter: &mut Interpreter, contents: &str) -> Result<(), String> {
    let mut tokenizer = Tokenizer::new(contents);
    let tokens = tokenizer.tokenize()?;
    //println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    let expr = parser.parse()?;
    //expr.print();
    let result = interpreter.interpret(expr)?;
    
    println!("{}", result.to_string());

    return Ok(());
}

fn run_file(path: &str) -> Result<(), String> {
    let mut interpreter = Interpreter::new();

    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&mut interpreter, &contents),
    }
}

fn run_prompt() -> Result<(), String> {
    let mut interpreter = Interpreter::new();

    loop {
        print!(">>> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("Couldn't flush stdout".to_string()),
        }

        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(n) => {
                if n <= 2 {
                    return Ok(());
                }
            },
            Err(_) => return Err("Couldn't read line".to_string()),
        }

        //println!("Echo: {}", buffer);
        match run(&mut interpreter, &buffer) {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: \"corrode [file]\"");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("Error:\n{}", msg);
                exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("Error:\n{}", msg);
                exit(1);
            }
        }
    }
}