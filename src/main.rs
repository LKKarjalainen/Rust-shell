use std::io::{self, Write};
use std::process::{Command, exit};

mod builtin;
mod path;

fn main() {
    loop {
        print!("$ ");
        let mut input: String = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let mut input_vec: Vec<&str> = input.trim().split(' ').collect();
        let args: Vec<&str> = input_vec.split_off(1);
        let command: &str = input_vec[0].trim();
        if command == "" {
            continue;
        }
        //println!("command: {:?}", input_vec);
        //println!("args: {:?}", args);

        match builtin::Builtin::from_name(command) {
            Some(builtin) => {
                if let builtin::Loop::Exit(code) = builtin.run(&args) {
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
