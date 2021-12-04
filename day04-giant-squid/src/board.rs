use std::collections::HashMap;

#[derive(Debug)]
pub struct Board {
    numbers: HashMap<i32, (usize, usize)>,
    marked: HashMap<(usize, usize), (i32, bool)>,
    pub unmarked_sum: i32,
    cols: usize,
    rows: usize,
}

impl Board {
    pub fn new() -> Self {
        Board {
            numbers: HashMap::new(),
            marked: HashMap::new(),
            unmarked_sum: 0,
            cols: 5,
            rows: 5,
        }
    }

    pub fn add_row(&mut self, row_items: Vec<i32>, row: usize) {
        for col in 0..self.cols {
            self.add_number(row_items[col], row, col)
        }
    }

    pub fn add_number(&mut self, num: i32, row: usize, col: usize) {
        let pos = (col, row);
        self.numbers.insert(num, pos);
        self.marked.insert(pos, (num, false));
        self.unmarked_sum += num;
    }

    pub fn mark(&mut self, num: i32) -> bool {
        if !self.numbers.contains_key(&num) {
            return false;
        }

        let pos = self.numbers[&num];
        self.marked.insert(pos, (num, true));
        self.unmarked_sum -= num;

        self.has_won(pos.0, pos.1)
    }

    fn has_won(&self, col: usize, row: usize) -> bool {
        let mut row_marked = true;
        for i in 0..self.cols {
            if !self.marked[&(i, row)].1 {
                row_marked = false;
            }
        }
        let mut col_marked = true;
        for j in 0..self.rows {
            if !self.marked[&(col, j)].1 {
                col_marked = false;
            }
        }
        row_marked || col_marked
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                result += &format!(
                    "{:>4}{}",
                    self.marked[&(col, row)].0,
                    if self.marked[&(col, row)].1 { "*" } else { " " },
                );
            }
            result += "\n";
        }
        write!(f, "{}", result)
    }
}
