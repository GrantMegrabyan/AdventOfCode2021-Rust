use shared::{file_lines, read_first_arg, MyError};

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    calculate_power_consumption(&input)?;

    Ok(())
}

fn calculate_power_consumption(file_path: &str) -> Result<(), MyError> {
    let mut ones_count: Vec<i32> = vec![];
    let mut total = 0;
    for line in file_lines(file_path)? {
        parse_line(&mut ones_count, &line.unwrap());
        total += 1;
    }

    let half = total / 2;
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    for i in 0..ones_count.len() {
        let d = 1 << ones_count.len() - 1 - i;
        if ones_count[i] > half {
            gamma_rate += d;
        } else {
            epsilon_rate += d;
        }
    }

    println!("Gamma rate = {}, Epsilon rate = {}", gamma_rate, epsilon_rate);
    println!("Power consumption: {}", gamma_rate * epsilon_rate);

    Ok(())
}

fn parse_line(ones_count: &mut Vec<i32>, line: &str) {
    if ones_count.is_empty() {
        ones_count.append(&mut vec![0; line.len()]);
    }
    let mut i = 0;
    for c in line.chars() {
        ones_count[i] += c as i32 - '0' as i32;
        i += 1;
    }
}
