mod line;

use line::Line;
use shared::{file_lines, read_first_arg, MyError};
use std::collections::{HashMap, hash_map::Entry};

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    count_dangerous_points(&input)?;

    Ok(())
}

fn count_dangerous_points(file_path: &str) -> Result<(), MyError> {
    let mut field: HashMap<(i32, i32), i32> = HashMap::new();
    let mut danger_counter = 0;

    for input_line in file_lines(file_path)? {
        let line = Line::from_input(&input_line.unwrap());
        if !line.is_parallel() {
            continue;
        }
        for point in line.iter() {
            match field.entry(point) {
                Entry::Vacant(v) => {
                    v.insert(1);
                },
                Entry::Occupied(o) => {
                    if *o.get() == 1 {
                        danger_counter += 1;
                    }
                    *(o.into_mut()) += 1;
                }
            }
        }
    }
    println!("Danger counter: {}", danger_counter);
    Ok(())
}
