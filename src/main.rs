#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::fs::{self, Metadata};
use std::process::Command;


const BUILT_IN: &[&str] = &["echo", "type", "exit"];

fn search_path(command: &str) -> Option<String> {
    let path: String = env::var("PATH").ok()?;
    let path_dirs:Vec<&str> = path.split(':').collect();
    for path_dir in path_dirs {
        let command_path: String = format!("{}/{}", path_dir, command);
        if Path::new(&command_path).exists() {
            let metadata: Metadata = fs::metadata(&command_path).unwrap();
            if metadata.mode() & 0o111 != 0 {
                return Some(command_path);
            }
        }
    }
    None
}   

fn main() {
    loop {
        print!("$ ");
        let mut input:String = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let mut input_vec:Vec<&str> = input.trim().split(' ').collect();
        let args: Vec<&str> = input_vec.split_off(1);
        let command: &str = input_vec[0].trim();
        println!("command: {:?}", input_vec);
        println!("args: {:?}", args);
        if command == "type" {
            let mut output: String = String::new();
            output.push_str(args[0].trim());
            if BUILT_IN.contains(&args[0].trim()) {
                output.push_str(" is a shell builtin");
            }
            else if let Some(path) = search_path(&args[0].trim()) {
                output.push_str(" is ");
                output.push_str(&path);
            }
            else {
                output.push_str(": not found");
            }
            println!("{}", output);
            continue;
        }
        if command == "echo" {
            let mut output: String = String::new();
            for arg in args {
                output.push_str(arg.trim());
                output.push(' ');
            }
            println!("{}", output);
            continue;
        }
        if command == "exit" {
            break;
        }
        if let Some(path) = search_path(&command) {
            //println!("executable in {}", path);
            let exec_output = Command::new(&command).args(args).output().unwrap();
            //println!("{:?}", exec_output);
            if exec_output.stdout.is_empty() {
                print!("{}", String::from_utf8_lossy(&exec_output.stderr));
            }
            else {
                print!("{}", String::from_utf8_lossy(&exec_output.stdout))
            }
            continue;
        }
        println!("{}: command not found", command);
    }
}
