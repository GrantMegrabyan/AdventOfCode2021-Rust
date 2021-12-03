use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use shared::{read_first_arg, MyError};

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;
    count_increases(&input)?;

    Ok(())
}

fn count_increases(file_path: &str) -> Result<(), MyError> {
    let f = File::open(file_path)?;
    let f = BufReader::new(f);

    let mut prev: Option<i32> = None;
    let mut counter: usize = 0;
    for line in f.lines() {
        let depth: i32 = line.unwrap().parse().unwrap();
        if let Some(prev_depth) = prev {
            if depth > prev_depth {
                counter += 1;
            }
        }
        prev = Some(depth);
    }
    println!("Number of depth increases: {}", counter);
    Ok(())
}
