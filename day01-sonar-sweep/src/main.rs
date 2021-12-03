mod sliding_window;

use shared::{read_first_arg, MyError};
use sliding_window::SlidingWindow;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    count_increases(&input)?;
    count_increases_sliding_window(&input)?;

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

fn count_increases_sliding_window(file_path: &str) -> Result<(), MyError> {
    let f = File::open(file_path)?;
    let f = BufReader::new(f);

    let window_size: usize = 3;

    let mut counter: usize = 0;
    let mut window1 = SlidingWindow::new(window_size);
    let mut window2 = SlidingWindow::new(window_size);

    let mut lines = f.lines();
    if let Some(line) = lines.next() {
        let depth: i32 = line.unwrap().parse().unwrap();
        window1.add(depth)
    }

    for line in lines {
        let depth: i32 = line.unwrap().parse().unwrap();
        window2.add(depth);
        if window2.is_full() {
            if window2.sum > window1.sum {
                counter += 1;
            }
        }
        window1.add(depth);
    }
    println!(
        "Number of depth increases with window size {}: {}",
        window_size, counter
    );
    Ok(())
}
