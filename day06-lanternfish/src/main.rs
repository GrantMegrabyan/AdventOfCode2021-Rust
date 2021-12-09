use shared::{file_lines, read_first_arg, MyError};

fn main() -> Result<(), MyError> {
    let file_path = read_first_arg()?;
    let fish = file_lines(&file_path)?
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let population = count_population(fish, 80)?;
    println!("Fish population after 80 days: {}", population);

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
}
