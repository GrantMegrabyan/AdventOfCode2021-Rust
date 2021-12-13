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

    let fuel = calculate_min_fuel_consuption(crabs.clone())?;
    println!("Min fuel consumption in constant rate: {}", fuel);

    let fuel = calculate_min_fuel_consuption_avg(crabs)?;
    println!("Min fuel consumption in increasing rate: {}", fuel);

    Ok(())
}

fn calculate_min_fuel_consuption(mut crabs: Vec<i32>) -> Result<i32, MyError> {
    crabs.sort();
    let median = crabs.get(crabs.len() / 2).unwrap();
    let result = crabs.iter()
        .fold(0, |acc, crab| acc + (median - crab).abs());
    Ok(result)
}

fn calculate_min_fuel_consuption_avg(crabs: Vec<i32>) -> Result<i32, MyError> { 
    let avg = crabs.iter().sum::<i32>() / crabs.len() as i32;
    let result = (avg..avg+2)
        .map(|m| {
            crabs.iter()
                .fold(0, |acc, crab| acc + ((m - crab).abs() * (1 + (m - crab).abs()))/2)
        })
        .min()
        .unwrap();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_min_fuel_consuption() {
        let fuel = calculate_min_fuel_consuption(vec![16,1,2,0,4,2,7,1,2,14]).unwrap();
        assert_eq!(37, fuel);

        let fuel = calculate_min_fuel_consuption_avg(vec![16,1,2,0,4,2,7,1,2,14]).unwrap();
        assert_eq!(168, fuel);
    }
}