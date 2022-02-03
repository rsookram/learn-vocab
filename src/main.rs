use std::env;

fn main() {
    let mut args = env::args();
    args.next(); // skip program name

    let file_path = args.next().unwrap();
    let db_path = args.next().unwrap();

    println!("{}, {}", file_path, db_path);
}
