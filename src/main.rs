use std::io::Read;

fn main() {
    let mut contents = String::new();
    for arg in &std::env::args().collect::<Vec<String>>()[1..] {
        std::fs::File::open(&arg)
            .expect("Le fichier n'existe pas")
            .read_to_string(&mut contents)
            .expect("Impossible de lire");
        println!("{}", contents);
    }
}