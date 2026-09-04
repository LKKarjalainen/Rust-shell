use crate::path;

use std::env;
use std::path::Path;

pub enum Loop {
    Continue,
    Exit(i32),
}

#[derive(Debug, Clone, Copy)]
pub enum Builtin {
    Echo,
    Type,
    Exit,
    Pwd,
    Cd,
}

impl Builtin {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "echo" => Some(Self::Echo),
            "type" => Some(Self::Type),
            "exit" => Some(Self::Exit),
            "pwd" => Some(Self::Pwd),
            "cd" => Some(Self::Cd),
            _ => None,
        }
    }

    pub fn run(&self, args: &[String]) -> Loop {
        match self {
            Self::Echo => {
                println!("{}", args.join(" "));
                return Loop::Continue;
            }
            Self::Type => {
                let mut output: String = String::new();
                let Some(first_arg) = args.first() else {
                    return Loop::Continue;
                };
                output.push_str(first_arg);
                if Builtin::from_name(first_arg).is_some() {
                    output.push_str(" is a shell builtin");
                } else if let Some(path) = path::search_path(first_arg) {
                    output.push_str(" is ");
                    output.push_str(&path);
                } else {
                    output.push_str(": not found");
                }
                println!("{}", output);
                return Loop::Continue;
            }
            Self::Pwd => {
                let path: std::path::PathBuf = env::current_dir().unwrap();
                println!("{}", path.as_path().to_string_lossy());
                return Loop::Continue;
            }
            Self::Cd => {
                if args.is_empty() {
                    println!("You must provide a path!");
                    return Loop::Continue;
                }
                if args[0] == "~" {
                    env::set_current_dir(env::home_dir().unwrap()).unwrap();
                    return Loop::Continue;
                }
                let path: &Path = Path::new(&args[0]);
                let exists = path.exists();
                if !exists {
                    println!("cd: {}: No such file or directory", path.to_string_lossy());
                    return Loop::Continue;
                }
                env::set_current_dir(path).unwrap();
                return Loop::Continue;
            }
            Self::Exit => {
                if let Some(code) = args.first() {
                    let code: i32 = code.parse().unwrap_or(67);
                    return Loop::Exit(code);
                }
                return Loop::Exit(67);
            }
        }
    }
}
