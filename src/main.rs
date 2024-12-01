use std::io::{self, Read};

pub mod d1;

fn load_data(path: &str) -> io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.trim().to_string();
    Ok(contents)
}

fn main() {
    d1::day1();
}
