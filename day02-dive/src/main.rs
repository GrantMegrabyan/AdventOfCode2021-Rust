use shared::{file_lines, read_first_arg, MyError};

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    calculate_position(&input)?;
    calculate_position_with_aim(&input)?;

    Ok(())
}

fn calculate_position(file_path: &str) -> Result<(), MyError> {
    let mut horiz = 0;
    let mut vert = 0;

    for line in file_lines(file_path)? {
        match parse(&line.unwrap()) {
            ("forward", val) => horiz += val,
            ("down", val) => vert += val,
            ("up", val) => vert -= val,
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

fn calculate_position_with_aim(file_path: &str) -> Result<(), MyError> {
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in file_lines(file_path)? {
        match parse(&line.unwrap()) {
            ("forward", val) => {
                horiz += val;
                depth += aim * val;
            }
            ("down", val) => aim += val,
            ("up", val) => aim -= val,
            _ => eprintln!("Invalid command"),
        }
    }

    println!(
        "Horizontal({}) x Depth({}) = {}",
        horiz,
        depth,
        horiz * depth,
    );

    Ok(())
}

fn parse(line: &str) -> (&str, i32) {
    line.split_once(" ")
        .map(|s| (s.0, s.1.parse::<i32>().unwrap()))
        .unwrap()
}
