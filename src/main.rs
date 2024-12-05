use std::io::{self, Read};

pub mod d1;
pub mod d2;
pub mod d3;
pub mod d4;
pub mod d5;

fn load_data(path: &str) -> io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.trim().to_string();
    Ok(contents)
}

fn main() {
    println!("Day1");
    d1::day1();
    println!("Day2");
    d2::day2();
    println!("Day3");
    d3::day3();
    println!("Day4");
    d4::day4();
    println!("Day5");
    d5::day5();
}
