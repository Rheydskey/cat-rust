use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
     let mut contents = String::new();
    for arg in &args[1..] {
        let mut _fichier = match File::open(&arg){
            Ok(_f) => {       
        let mut _file = File::open(&arg).expect("The file is not found").read_to_string(&mut contents).expect("I can't read the file");
        println!("{}", contents);
            }
            Err(_e) => {
                println!("Le fichier n'existe pas")
            }
        };

}
}
