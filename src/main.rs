#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        let mut input:String = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        if input == "exit" {
            break;
        }
        println!("{}: command not found", input.trim());
    }
}
