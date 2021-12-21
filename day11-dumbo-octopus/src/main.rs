extern crate ansi_escapes;

use ansi_term::Style;
use shared::{file_lines, read_first_arg, MyError};
use std::collections::VecDeque;
use std::fmt::Display;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), MyError> {
    let file_path = read_first_arg()?;
    let initial_state: Vec<Vec<u8>> = file_lines(&file_path)
        .unwrap()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|ch| ch as u8 - '0' as u8)
                .collect::<Vec<_>>()
        })
        .collect();

    let steps = 100;
    let grid = Grid::new(initial_state.clone(), false);
    let flashes = count_flashes(grid, steps);
    println!("Total falshes in {} steps: {}", steps, flashes);

    let grid = Grid::new(initial_state, true);
    let all_flashed_step = get_step_when_all_flashed(grid);
    println!("All octopus will flash on step {}", all_flashed_step);

    Ok(())
}

fn count_flashes(mut grid: Grid, steps: usize) -> usize {
    for _ in 0..steps {
        grid.tick();
    }
    grid.flash_counter
}

fn get_step_when_all_flashed(mut grid: Grid) -> usize {
    while !grid.all_flashed() {
        grid.tick();
    }
    grid.step
}

type Coord = (usize, usize);

struct Grid {
    grid: Vec<Vec<u8>>,
    pub step: usize,
    pub flash_counter: usize,
    flash_counter_on_step: usize,
    print_steps: bool,
}

impl Grid {
    pub fn new(initial_state: Vec<Vec<u8>>, print_steps: bool) -> Self {
        Grid {
            grid: initial_state,
            step: 0,
            flash_counter: 0,
            flash_counter_on_step: 0,
            print_steps,
        }
    }

    pub fn tick(&mut self) {
        self.step += 1;
        self.flash_counter_on_step = 0;

        let mut ready_to_flash = self.increase_energy_levels();
        self.print();

        while !ready_to_flash.is_empty() {
            let octopus = ready_to_flash.pop_front().unwrap();
            for oct in self.flash(octopus) {
                ready_to_flash.push_back(oct)
            }
            self.print();
        }

        self.reset_flashed();
        self.print();
    }

    pub fn all_flashed(&self) -> bool {
        self.flash_counter_on_step == self.grid.len() * self.grid[0].len()
    }

    fn increase_energy_levels(&mut self) -> VecDeque<Coord> {
        let mut ready_to_flash = VecDeque::new();

        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                self.grid[row][col] += 1;
                if self.grid[row][col] == 10 {
                    ready_to_flash.push_back((row, col));
                    self.flash_counter_on_step += 1;
                }
            }
        }

        ready_to_flash
    }

    const ADJACENT: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    fn flash(&mut self, cur: Coord) -> Vec<Coord> {
        Self::ADJACENT
            .iter()
            .map(|(xx, yy)| {
                (
                    (cur.0 as isize + xx) as usize,
                    (cur.1 as isize + yy) as usize,
                )
            })
            .filter_map(
                |(row, col)| match &self.grid.get(row).and_then(|l| l.get(col)) {
                    Some(_) => {
                        self.grid[row][col] += 1;
                        if self.grid[row][col] == 10 {
                            self.flash_counter_on_step += 1;
                            Some((row, col))
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
            )
            .collect()
    }

    fn reset_flashed(&mut self) {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if self.grid[row][col] > 9 {
                    self.grid[row][col] = 0;
                }
            }
        }
        self.flash_counter += self.flash_counter_on_step;
    }

    fn print(&self) {
        if self.print_steps {
            print!("{}", self);
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        sleep(Duration::from_millis(20));
        let mut s = format!("{}", ansi_escapes::EraseLines(self.grid.len() as u16 + 3));
        s += format!(
            "Step: {}, Flashes: {}\n\n",
            self.step,
            self.flash_counter + self.flash_counter_on_step
        )
        .as_str();

        for row in &self.grid {
            s += row
                .iter()
                .map(|&a| {
                    if a > 9 {
                        format!("{} ", Style::new().bold().paint("0"))
                    } else {
                        format!("{} ", a)
                    }
                })
                .collect::<String>()
                .as_str();
            s += "\n";
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_flashes() {
        let mut grid = Grid::new(
            vec![
                vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
                vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
                vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
                vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
                vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
                vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
                vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
                vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
                vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
                vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
            ],
            false,
        );
        for _ in 0..100 {
            grid.tick();
        }
        let expected = 1656;
        assert_eq!(expected, grid.flash_counter);
    }
}
