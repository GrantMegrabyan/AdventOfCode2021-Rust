use std::collections::{HashSet, VecDeque, BinaryHeap};
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

    let largest_basins = find_largest_basins(&input);
    println!("Largest basins: {}", largest_basins);

    Ok(())
}

fn calculate_risk_level(heightmap: &Vec<Vec<u32>>) -> u32 {
    get_low_points(heightmap)
        .iter()
        .map(|(row_idx, col_idx)| &heightmap[*row_idx][*col_idx] + 1)
        .sum()
}

fn find_largest_basins(heightmap: &Vec<Vec<u32>>) -> u32 {
    let mut heap = BinaryHeap::new();

    get_low_points(heightmap)
        .iter()
        .map(|&p| calculate_basin_size(heightmap, p))
        .for_each(|size| heap.push(size));

    (0..3)
        .map(|_| heap.pop().unwrap_or(1))
        .product()
}

fn calculate_basin_size(heightmap: &Vec<Vec<u32>>, start: Point) -> u32 {
    let max_row = heightmap.len() - 1;
    let max_col = heightmap[0].len() - 1;
    let get_adj = |p: Point| get_adj_idx(max_row, max_col, p.0, p.1);

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut visited: HashSet<Point> = HashSet::new();

    let mut size = 0;
    while !queue.is_empty() {
        let point = queue.pop_front().unwrap();
        visited.insert(point);
        size += 1;

        get_adj(point)
            .iter()
            .filter(|&p| {
                heightmap[p.0][p.1] != 9 && heightmap[p.0][p.1] > heightmap[point.0][point.1]
            })
            .for_each(|&p| {
                if !visited.contains(&p) {
                    visited.insert(p);
                    queue.push_back(p);
                }
            });
    }
    size
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

    #[test]
    fn test_calculate_basin_size() {
        let heightmap: Vec<Vec<u32>> = "
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678"
            .split_ascii_whitespace()
            .filter(|&s| !s.is_empty())
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let expected = 3;
        let actual = calculate_basin_size(&heightmap, (0, 1));
        assert_eq!(expected, actual);

        let expected = 9;
        let actual = calculate_basin_size(&heightmap, (0, 9));
        assert_eq!(expected, actual);

        let expected = 14;
        let actual = calculate_basin_size(&heightmap, (2, 2));
        assert_eq!(expected, actual);

        let expected = 9;
        let actual = calculate_basin_size(&heightmap, (4, 6));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_largest_basins() {
        let heightmap: Vec<Vec<u32>> = "
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678"
            .split_ascii_whitespace()
            .filter(|&s| !s.is_empty())
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let expected = 1134;
        let actual = find_largest_basins(&heightmap);
        assert_eq!(expected, actual);
    }
}
