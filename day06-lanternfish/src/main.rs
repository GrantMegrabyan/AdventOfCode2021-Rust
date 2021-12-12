use shared::{file_lines, read_first_arg, MyError};
use std::collections::HashMap;

fn main() -> Result<(), MyError> {
    let file_path = read_first_arg()?;
    let fish = file_lines(&file_path)?
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let population = count_population(fish.clone(), 80)?;
    println!("Fish population after 80 days: {}", population);

    let population = count_population_with_memoization(fish, 256)?;
    println!("Fish population after 256 days: {}", population);

    Ok(())
}

fn count_population(mut fish: Vec<usize>, days: usize) -> Result<usize, MyError> {
    let mut newbies: Vec<usize> = vec![];
    for _ in 0..days {
        for f in fish.iter_mut() {
            if *f == 0 {
                newbies.push(8);
                *f = 6;
            } else {
                *f -= 1;
            }
        }
        fish.append(&mut newbies);
    }
    Ok(fish.len())
}

type MemoKey = (usize, i32);

fn count_population_with_memoization(fish: Vec<usize>, days: i32) -> Result<usize, MyError> {
    let mut memo: HashMap<MemoKey, usize> = HashMap::new();

    let population = fish
        .into_iter()
        .fold(0, |acc, f| acc + population_in_days(f, days, &mut memo));

    Ok(population)
}

fn population_in_days(age: usize, days: i32, memo: &mut HashMap<MemoKey, usize>) -> usize {
    if age as i32 > days - 1 {
        1
    } else {
        let remaining_days = std::cmp::max(0, days - age as i32 - 1);
        memo_get_or_insert(memo, (6, remaining_days))
            + memo_get_or_insert(memo, (8, remaining_days))
    }
}

fn memo_get_or_insert(memo: &mut HashMap<MemoKey, usize>, key: MemoKey) -> usize {
    if memo.contains_key(&key) {
        *memo.get(&key).unwrap()
    } else {
        let result = population_in_days(key.0, key.1, memo);
        memo.insert(key, result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_population() {
        let fish = vec![3, 4, 3, 1, 2];

        let actual = count_population(fish.clone(), 18).unwrap();
        let expected = 26;

        assert_eq!(expected, actual);

        let actual = count_population(fish, 80).unwrap();
        let expected = 5934;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_population_with_memoization() {
        let fish = vec![3, 4, 3, 1, 2];

        let actual = count_population_with_memoization(fish.clone(), 18).unwrap();
        let expected = 26;

        assert_eq!(expected, actual);

        let actual = count_population_with_memoization(fish, 80).unwrap();
        let expected = 5934;

        assert_eq!(expected, actual);
    }
}