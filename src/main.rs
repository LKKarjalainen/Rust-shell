#[allow(unused_imports)]
use std::io::{self, Write};

const BUILT_IN: &[&str] = &["echo", "type", "exit"];

fn main() {
    loop {
        print!("$ ");
        let mut input:String = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let mut input_vec:Vec<&str> = input.split(' ').collect();
        let args: Vec<&str> = input_vec.split_off(1);
        let command: &str = input_vec[0];
        //println!("command: {:?}", input_vec);
        //println!("args: {:?}", args);
        if command.trim() == "type" {
            let mut output: String = String::new();
            output.push_str(args[0].trim());
            if BUILT_IN.contains(&args[0].trim()) {
                output.push_str(" is a shell builtin");
            }
            else {
                output.push_str(": not found");
            }
            println!("{}", output);
            continue;
        }
        if command.trim() == "echo" {
            let mut output: String = String::new();
            for arg in args {
                output.push_str(arg.trim());
                output.push(' ');
            }
            println!("{}", output);
            continue;
        }
        if command.trim() == "exit" {
            break;
        }
        println!("{}: command not found", command.trim());
    }
}
