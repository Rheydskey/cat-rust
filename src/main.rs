use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    for arg in &args[1..] {
        let mut contents = String::new();
        let mut file = File::open(&arg)?.read_to_string(&mut contents)?;
        println!("{}", contents);
    }
    Ok(())
}