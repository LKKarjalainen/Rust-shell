use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
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
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        // Capturing input ends

        // Lexing begins
        let words: Vec<String> = lexer::lex(&input);
        println!("{:?}", words);
        // Lexing ends
        
        
        // Create Writer
        let mut writer: Box<dyn Write> = Box::new(io::stdout());
        let mut words_iter = words.iter();
        while let Some(word) = words_iter.next() {
            match word.as_str() {
                ">" | "1>" => {
                    if let Some(path_str) = words_iter.next() {
                        let path: &Path =  Path::new(path_str);
                        let file: File = File::create_new(path).expect("File creation failed");
                        writer = Box::new(file);
                    }
                    else {
                        println!("No file provided for redirection.");
                        break;
                    }
                }

                _ => {}
            }
        }
        // Writer created
        
        let Some((command, args)) = words.split_first() else {
            continue;
        };
        //println!("command: {:?}", command);
        //println!("args: {:?}", args);
        
        // Handling input begins
        match builtin::Builtin::from_name(command) {
            Some(builtin) => {
                if let builtin::Loop::Exit(code) = builtin.run(args, writer) {
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
