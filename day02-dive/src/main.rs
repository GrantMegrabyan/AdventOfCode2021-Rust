use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use shared::{read_first_arg, MyError};

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    calculate_position(&input)?;

    Ok(())
}

fn calculate_position(file_path: &str) -> Result<(), MyError> {
    let f = File::open(file_path)?;
    let f = BufReader::new(f);

    let mut horiz = 0;
    let mut vert = 0;

    for line in f.lines() {
        match line.unwrap().split_once(" ") {
            Some(("forward", val)) => horiz += val.parse::<i32>().unwrap(),
            Some(("down", val)) => vert += val.parse::<i32>().unwrap(),
            Some(("up", val)) => vert -= val.parse::<i32>().unwrap(),
            _ => eprintln!("Invalid command"),
        }
    }

    println!(
        "Horizontal({}) x Vertical({}) = {}",
        horiz,
        vert, 
        horiz * vert,
    );

    Ok(())
}