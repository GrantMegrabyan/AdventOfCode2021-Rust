pub struct Line {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Line {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Line { x1, y1, x2, y2 }
    }

    pub fn from_input(input: &str) -> Self {
        let coords: Vec<i32> = input
            .replace(" -> ", ",")
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        Line::new(coords[0], coords[1], coords[2], coords[3])
    }

    pub fn is_parallel(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    pub fn iter(&self) -> LineIter {
        LineIter::new(self)
    }
}

pub struct LineIter<'a> {
    line: &'a Line,
    curr: Option<(i32, i32)>,
    direction: (i32, i32),
}

impl<'a> LineIter<'a> {
    pub fn new(line: &'a Line) -> Self {
        let x_dir = if line.x1 == line.x2 {
            0
        } else {
            (line.x2-line.x1) / (line.x2-line.x1).abs()
        };
        let y_dir = if line.y1 == line.y2 {
            0
        } else {
            (line.y2-line.y1) / (line.y2-line.y1).abs()
        };
        let direction = (x_dir, y_dir);

        Self {
            line,
            curr: None,
            direction,
        }
    }
}

impl<'a> Iterator for LineIter<'a> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr {
            Some(curr) => {
                if curr == (self.line.x2, self.line.y2) {
                    self.curr = None;
                } else {
                    let new_curr = (curr.0 + self.direction.0, curr.1 + self.direction.1);
                    self.curr = Some(new_curr)
                }
            },
            None => {
                self.curr = Some((self.line.x1, self.line.y1))
            }
        }
        self.curr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_is_parallel() {
        let line = Line::new(1, 5, 1, 10);
        assert_eq!(true, line.is_parallel());

        let line = Line::new(10, 5, 1, 5);
        assert_eq!(true, line.is_parallel());

        let line = Line::new(10, 50, 1, 5);
        assert_eq!(false, line.is_parallel());
    }

    #[test]
    fn test_line_from_input() {
        let input = "0,9 -> 5,9";
        let line = Line::from_input(input);

        assert_eq!(0, line.x1);
        assert_eq!(9, line.y1);
        assert_eq!(5, line.x2);
        assert_eq!(9, line.y2);
    }
}
