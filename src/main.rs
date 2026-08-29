#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::fs::{self, Metadata};
use std::process::{Command, exit};


enum Loop {
    Continue,
    Exit(i32),
}

#[derive(Debug, Clone, Copy)]
enum Builtin {
    Echo,
    Type,
    Exit,
    Pwd,
    Cd,
}

impl Builtin {
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "echo" => Some(Self::Echo),
            "type" => Some(Self::Type),
            "exit" => Some(Self::Exit),
            "pwd" => Some(Self::Pwd),
            "cd" => Some(Self::Cd),
            _ => None,
        }
    }

    fn run(&self, args: &[&str]) -> Loop {
        match self {
            Self::Echo => {
                println!("{}", args.join(" "));
                return Loop::Continue;
            },
            Self::Type => {
                let mut output: String = String::new();
                let Some(first_arg) = args.first() else { return Loop::Continue; };
                output.push_str(first_arg);
                if Builtin::from_name(first_arg).is_some() {
                    output.push_str(" is a shell builtin");
                }
                else if let Some(path) = search_path(first_arg) {
                    output.push_str(" is ");
                    output.push_str(&path);
                }
                else {
                    output.push_str(": not found");
                }
                println!("{}", output);
                return Loop::Continue;
            },
            Self::Pwd => {
                let path: std::path::PathBuf =  env::current_dir().unwrap();
                println!("{}", path.as_path().to_string_lossy());
                return Loop::Continue;
            },
            Self::Cd => {
                if args.is_empty() {
                    println!("You must provide a path!");
                    return Loop::Continue;
                }
                if args[0] == "~" {
                    env::set_current_dir(env::home_dir().unwrap()).unwrap();
                    return Loop::Continue;
                }
                let path: &Path = Path::new(args[0]);
                let exists = path.exists();
                if !exists {
                    println!("cd: {}: No such file or directory", path.to_string_lossy());
                    return Loop::Continue;
                }
                env::set_current_dir(path).unwrap();
                return Loop::Continue;
            },
            Self::Exit => { return Loop::Exit(67); }
        }
    }
}

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
        if command == "" {
            continue;
        }
        //println!("command: {:?}", input_vec);
        //println!("args: {:?}", args);

        match Builtin::from_name(command) {
            Some(builtin) => {
                if let Loop::Exit(code) = builtin.run(&args) {
                    exit(code);
                }
                continue;
            }
            None => {
                if let Some(path) = search_path(command) {
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
            }
        }
        
        println!("{}: command not found", command);
    }
}
