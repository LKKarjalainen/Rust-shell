use std::io::Read;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    let mut input:String = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)?;
    println!("{}", input);
}
