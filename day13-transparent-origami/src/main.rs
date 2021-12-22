use shared::{file_lines, read_first_arg, MyError};
use std::{collections::HashSet, fmt::Display};

fn main() -> Result<(), MyError> {
    let file_path = read_first_arg()?;
    let mut grid: HashSet<Point> = HashSet::new();
    let mut commands: Vec<(FoldAxis, usize)> = vec![];

    file_lines(&file_path).unwrap().for_each(|l| {
        let l = l.unwrap();
        if l.starts_with("fold along") {
            match l.splitn(3, " ").last().unwrap().split_once("=").unwrap() {
                ("x", pos) => commands.push((FoldAxis::X, pos.parse().unwrap())),
                ("y", pos) => commands.push((FoldAxis::Y, pos.parse().unwrap())),
                _ => unreachable!(),
            }
        } else if !l.is_empty() {
            match l.split_once(",").unwrap() {
                (x, y) => {
                    grid.insert(Point(x.parse().unwrap(), y.parse().unwrap()));
                }
            }
        }
    });

    let mut paper = Paper::new(grid);
    match commands[0] {
        (FoldAxis::Y, pos) => paper.fold_y(pos),
        (FoldAxis::X, pos) => paper.fold_x(pos),
    }

    println!("Dots after first fold: {}", paper.points_count());

    Ok(())
}

#[derive(Debug)]
enum FoldAxis {
    X,
    Y,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point(usize, usize);

struct Paper {
    grid: HashSet<Point>,
    cols: usize,
    rows: usize,
}

impl Paper {
    pub fn new(grid: HashSet<Point>) -> Self {
        let cols = grid.iter().map(|p| p.0).max().unwrap();
        let rows = grid.iter().map(|p| p.1).max().unwrap();
        Paper { grid, cols, rows }
    }

    pub fn points_count(&self) -> usize {
        self.grid.len()
    }

    pub fn fold_y(&mut self, y: usize) {
        let points_to_fold = self
            .grid
            .iter()
            .filter_map(|p| if p.1 > y { Some(Point(p.0, p.1)) } else { None })
            .collect::<Vec<_>>();

        for point in points_to_fold {
            let folded = Point(point.0, 2 * y - point.1);
            if !self.grid.contains(&folded) {
                self.grid.insert(folded);
            }
            self.grid.remove(&point);
        }
    }

    pub fn fold_x(&mut self, pos: usize) {
        let points_to_fold = self
            .grid
            .iter()
            .filter_map(|p| {
                if p.0 > pos {
                    Some(Point(p.0, p.1))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for point in points_to_fold {
            let folded = Point(2 * pos - point.0, point.1);
            if !self.grid.contains(&folded) {
                self.grid.insert(folded);
            }
            self.grid.remove(&point);
        }
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 0..self.rows + 1 {
            for j in 0..self.cols + 1 {
                s += if self.grid.contains(&Point(i, j)) {
                    "#"
                } else {
                    "."
                };
            }
            s += "\n";
        }

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_fold() {
        let grid: HashSet<Point> = vec![
            Point(6, 10),
            Point(0, 14),
            Point(9, 10),
            Point(0, 3),
            Point(10, 4),
            Point(4, 11),
            Point(6, 0),
            Point(6, 12),
            Point(4, 1),
            Point(0, 13),
            Point(10, 12),
            Point(3, 4),
            Point(3, 0),
            Point(8, 4),
            Point(1, 10),
            Point(2, 14),
            Point(8, 10),
            Point(9, 0),
        ]
        .into_iter()
        .collect();

        let mut paper = Paper::new(grid);
        paper.fold_y(7);

        assert_eq!(17, paper.points_count());
    }
}
