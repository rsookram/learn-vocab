use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let mut args = env::args();
    args.next(); // skip program name

    let file_path = args.next().unwrap();
    let db_path = args.next().unwrap();

    let known_words = read_known_words(&file_path);

    println!("{}, {:?}", db_path, known_words);
}

fn read_known_words(path: &str) -> Vec<String> {
    let file = File::open(&path).unwrap();
    let mut known_words = Vec::new();

    let lines = io::BufReader::new(file).lines();
    lines.flat_map(|line| line).for_each(|line| {
        known_words.push(line);
    });

    known_words
}
