use shared::{file_lines, read_first_arg, MyError};

type Point = (usize, usize);

fn main() -> Result<(), MyError> {
    let file_path = read_first_arg()?;
    let input: Vec<Vec<u32>> = file_lines(&file_path)
        .unwrap()
        .map(|l| l.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let risk = calculate_risk_level(&input);
    println!("Rist level: {}", risk);

    Ok(())
}

fn calculate_risk_level(heightmap: &Vec<Vec<u32>>) -> u32 {
    get_low_points(heightmap)
        .iter()
        .map(|(row_idx, col_idx)| &heightmap[*row_idx][*col_idx] + 1)
        .sum()
}

fn get_low_points(heightmap: &Vec<Vec<u32>>) -> Vec<Point> {
    let max_row = heightmap.len() - 1;
    let max_col = heightmap[0].len() - 1;
    let get_adj = |row_idx: usize, col_idx: usize| get_adj_idx(max_row, max_col, row_idx, col_idx);

    heightmap
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(col_idx, item)| {
                    if get_adj(row_idx, col_idx)
                        .iter()
                        .all(|(adj_row_idx, adj_col_idx)| {
                            let adj = &heightmap[*adj_row_idx][*adj_col_idx];
                            item < adj
                        })
                    {
                        Some((row_idx, col_idx))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn get_adj_idx(max_row: usize, max_col: usize, row_idx: usize, col_idx: usize) -> Vec<Point> {
    let mut adj: Vec<Point> = vec![];
    if row_idx > 0 {
        adj.push((row_idx - 1, col_idx));
    }
    if col_idx > 0 {
        adj.push((row_idx, col_idx - 1));
    }
    if row_idx < max_row {
        adj.push((row_idx + 1, col_idx));
    }
    if col_idx < max_col {
        adj.push((row_idx, col_idx + 1));
    }
    adj
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_risk_level() {
        let input: Vec<Vec<u32>> = "
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678"
            .split_ascii_whitespace()
            .filter(|&s| !s.is_empty())
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        input.iter().for_each(|a: &Vec<u32>| println!("{:?}", a));
        println!("");

        let expected = 15;
        let actual = calculate_risk_level(&input);
        assert_eq!(expected, actual);
    }
}
