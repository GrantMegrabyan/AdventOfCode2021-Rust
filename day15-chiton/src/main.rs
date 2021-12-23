use std::collections::{BinaryHeap, HashMap};
use ansi_term::Colour::Green;
use shared::{file_lines, read_first_arg, MyError};

fn main() -> Result<(), MyError> {
    let file_path = read_first_arg()?;
    let grid: Vec<Vec<u8>> = file_lines(&file_path)
        .unwrap()
        .map(|l| l.unwrap().trim().bytes().map(|b| b - b'0').collect())
        .collect();

    let path = find_shortest_path(&grid).unwrap();
    println!("Shortest path is {}", path.value);
    visualise_path(&grid, &path);

    Ok(())
}

fn visualise_path(grid: &[Vec<u8>], path: &Path) {
    for (i, row) in grid.iter().enumerate() {
        for (j, node) in row.iter().enumerate() {
            if path.nodes.contains(&(i, j)) {
                print!("{}", Green.paint(node.to_string()))
            } else {
                print!("{}", node);
            }
        }
        println!();
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Path {
    nodes: Vec<(usize, usize)>, 
    last: (usize, usize),
    value: u64,
}

impl Path {
    pub fn add_node(&self, node: (usize, usize), weight: u8) -> Self {
        let mut nodes = self.nodes.clone();
        nodes.push(node);

        Path {
            nodes,
            last: node,
            value: self.value + weight as u64,
        }
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.value.cmp(&self.value)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_shortest_path(grid: &[Vec<u8>]) -> Option<Path> {
    let mut prio_queue: BinaryHeap<Path> = BinaryHeap::new();
    let path = Path {
        nodes: vec![(0, 0)],
        last: (0, 0),
        value: 0,
    };
    prio_queue.push(path);

    // Distances from "start" to given node
    let mut dist: HashMap<(usize, usize), u64> = HashMap::new();

    while let Some(path) = prio_queue.pop() {
        if path.last == (grid.len() - 1, grid[0].len() - 1) {
            return Some(path);
        }

        get_adjacent(grid, path.last)
            .iter()
            .for_each(|node| {
                let weight = grid[node.0][node.1];
                let dist_to_node = dist.entry(*node).or_insert(u64::MAX);
                let new_dist_to_node = weight as u64 + path.value;
                
                if new_dist_to_node < *dist_to_node {
                    // Found shorter path to the 'node"
                    prio_queue.push(path.add_node(*node, weight));
                    *dist_to_node = new_dist_to_node;
                }
            });
    }
    None
}

fn get_adjacent(grid: &[Vec<u8>], node: (usize, usize)) -> Vec<(usize, usize)> {
    [(-1, 0), (0, -1), (0, 1), (1, 0)]
        .iter()
        .map(|(xx, yy)| {
            (
                (node.0 as isize + xx) as usize,
                (node.1 as isize + yy) as usize,
            )
        })
        .filter_map(|(x, y)| grid.get(x).and_then(|a| a.get(y)).map(|_| (x, y)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path() {
        let input = r#"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
        "#;
        let grid: Vec<Vec<u8>> = input
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.trim().bytes().map(|b| b - b'0').collect())
            .collect();

        let path = find_shortest_path(&grid).unwrap();
        assert_eq!(40, path.value);
    }
}
