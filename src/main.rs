use std::io::{self, Write};
use std::process::{Command, exit};

mod builtin;
mod lexer;
mod path;

fn main() {
    loop {
        // Capturing input begins
        print!("$ ");
        let mut input: String = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        // Capturing input ends

        // Lexing begins
        let words: Vec<String> = lexer::lex(&input);
        //println!("{:?}", words);
        let Some((command, args)) = words.split_first() else {
            continue;
        };
        //println!("command: {:?}", command);
        //println!("args: {:?}", args);
        // Lexing ends

        // Handling input begins
        match builtin::Builtin::from_name(command) {
            Some(builtin) => {
                if let builtin::Loop::Exit(code) = builtin.run(args) {
                    exit(code);
                }
                continue;
            }
            None => {
                if path::search_path(command).is_some() {
                    //println!("executable in {}", path);
                    let exec_output = Command::new(&command).args(args).output().unwrap();
                    //println!("{:?}", exec_output);
                    if exec_output.stdout.is_empty() {
                        print!("{}", String::from_utf8_lossy(&exec_output.stderr));
                    } else {
                        print!("{}", String::from_utf8_lossy(&exec_output.stdout))
                    }
                    continue;
                }
            }
        }

        println!("{}: command not found", command);
    }
}
