#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

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
        if command.trim() == "echo" {
            let mut output: String = String::new();
            for arg in args {
                output.push_str(arg);
                output.push(' ');
            }
            print!("{}", output);
            continue;
        }
        if command.trim() == "exit" {
            break;
        }
        println!("{}: command not found", command.trim());
    }
}
