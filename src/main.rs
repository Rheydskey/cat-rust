use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
     let mut contents = String::new();
    for arg in &args[1..] {
       File::open(&arg).expect("Le fichier n'existe pas").read_to_string(&mut contents).expect("Impossible de lire");
       println!("{}", contents);
}
}
