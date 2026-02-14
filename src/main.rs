use std::env;
use std::fs;
use std::process;

fn main() {
    let mut args = env::args().skip(1);

    let filename = match args.next() {
        Some(name) => name,
        None => {
            eprintln!("Usage: rustpinky <filename>");
            process::exit(1);
        }
    };

    match fs::read_to_string(&filename) {
        Ok(source) => {
            println!("{}", source);
        }
        Err(err) => {
            eprintln!("Error reading {}: {}", filename, err);
            process::exit(1);
        }
    }
}
