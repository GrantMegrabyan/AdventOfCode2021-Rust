use shared::{file_lines, read_first_arg, MyError};

fn main() -> Result<(), MyError> {
    let file_path = read_first_arg()?;
    let crabs = file_lines(&file_path)?
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let fuel = calculate_min_fuel_consuption(crabs)?;
    println!("Min fuel consumption: {}", fuel);

    Ok(())
}

fn calculate_min_fuel_consuption(mut crabs: Vec<i32>) -> Result<i32, MyError> {
    crabs.sort();
    let median = crabs.get(crabs.len() / 2).unwrap();
    let result = crabs.iter()
        .fold(0, |acc, crab| acc + (median - crab).abs());
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_min_fuel_consuption() {
        let fuel = calculate_min_fuel_consuption(vec![16,1,2,0,4,2,7,1,2,14]).unwrap();
        assert_eq!(37, fuel);
    }
}