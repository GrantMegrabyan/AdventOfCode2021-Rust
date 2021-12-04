use shared::{file_lines, read_first_arg, MyError};
use std::cmp::Ordering;

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    calculate_power_consumption(&input)?;
    calculate_life_support(&input)?;

    Ok(())
}

fn calculate_power_consumption(file_path: &str) -> Result<(), MyError> {
    let (readings, bin_length) = get_readings(file_path)?;
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    let half = readings.len() / 2;
    for idx in (0..bin_length).rev() {
        let pattern = 1 << idx;
        let ones_cnt = readings.iter().filter(|&r| r & pattern != 0).count();
        if ones_cnt >= half {
            gamma_rate += pattern;
        } else {
            epsilon_rate += pattern;
        }
    }

    println!(
        "Gamma rate = {}, Epsilon rate = {}",
        gamma_rate, epsilon_rate
    );
    println!("Power consumption: {}", gamma_rate * epsilon_rate);

    Ok(())
}

fn calculate_life_support(file_path: &str) -> Result<(), MyError> {
    let (readings, bin_length) = get_readings(file_path)?;

    let oxygen_rate = get_life_support_rate(
        readings.clone(),
        bin_length,
        LifeSupportRatingType::MostCommon,
    );
    let co2_rate = get_life_support_rate(
        readings.clone(),
        bin_length,
        LifeSupportRatingType::LeastCommon,
    );

    println!("Oxygen rate = {}, CO2 rate = {}", oxygen_rate, co2_rate);
    println!("Life support rating: {}", oxygen_rate * co2_rate);

    Ok(())
}

fn get_life_support_rate(
    mut readings: Vec<i32>,
    bin_length: usize,
    rating_type: LifeSupportRatingType,
) -> i32 {
    for idx in (0..bin_length).rev() {
        let pattern = 1 << idx;
        let one_readings: Vec<i32> = readings
            .iter()
            .cloned()
            .filter(|r| r & pattern != 0)
            .collect();
        let zero_readings: Vec<i32> = readings
            .iter()
            .cloned()
            .filter(|r| r & pattern == 0)
            .collect();

        match (&rating_type, one_readings.len().cmp(&zero_readings.len())) {
            (LifeSupportRatingType::MostCommon, Ordering::Equal | Ordering::Greater) => {
                readings = one_readings
            }
            (LifeSupportRatingType::MostCommon, Ordering::Less) => readings = zero_readings,
            (LifeSupportRatingType::LeastCommon, Ordering::Less) => readings = one_readings,
            (LifeSupportRatingType::LeastCommon, Ordering::Equal | Ordering::Greater) => {
                readings = zero_readings
            }
        }

        if readings.len() == 1 {
            break;
        }
    }
    readings.first().unwrap().clone()
}

enum LifeSupportRatingType {
    MostCommon,
    LeastCommon,
}

fn get_readings(file_path: &str) -> Result<(Vec<i32>, usize), MyError> {
    let mut readings: Vec<i32> = vec![];
    let mut lines = file_lines(file_path)?;
    let first_line = lines.next().unwrap().unwrap();
    let bin_length = first_line.len();

    readings.push(i32::from_str_radix(&first_line, 2).unwrap());
    for line in lines {
        readings.push(i32::from_str_radix(&line.unwrap(), 2).unwrap());
    }
    Ok((readings, bin_length))
}
