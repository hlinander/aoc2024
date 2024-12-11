use std::io::{self, Read};

pub mod d1;
pub mod d10;
pub mod d11;
pub mod d2;
pub mod d3;
pub mod d4;
pub mod d5;
pub mod d6;
pub mod d7;
pub mod d8;
pub mod d9;

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
    println!("Day6");
    d6::day6();
    println!("Day7");
    d7::day7();
    println!("Day8");
    d8::day8();
    println!("Day9");
    d9::day9();
    println!("Day10");
    d10::day10();
    println!("Day11");
    d11::day11();
}
