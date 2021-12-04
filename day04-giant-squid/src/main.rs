mod board;
use board::Board;
use shared::{file_lines, read_first_arg, MyError};

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    let (boards, numbers) = read_input(&input)?;

    pick_first_to_win(boards.iter().cloned().collect(), numbers.clone())?;
    pick_last_to_win(boards, numbers)?;

    Ok(())
}

fn pick_first_to_win(mut boards: Vec<Board>, numbers: Vec<i32>) -> Result<(), MyError> {
    for num in numbers {
        print!("{} ", num);
        for i in 0..boards.len() {
            if boards[i].mark(num) {
                println!();
                println!("First board to win:");
                println!("{}", boards[i]);
                println!("Score = {}", num * boards[i].unmarked_sum);
                return Ok(());
            }
        }
    }
    Ok(())
}

fn pick_last_to_win(mut boards: Vec<Board>, numbers: Vec<i32>) -> Result<(), MyError> {
    for num in numbers {
        print!("{} ", num);
        for i in (0..boards.len()).rev() {
            if boards[i].mark(num) {
                if boards.len() == 1 {
                    println!();
                    println!("Last board to win:");
                    println!("{}", boards[i]);
                    println!("Score = {}", num * boards[i].unmarked_sum);
                    return Ok(());
                }
                boards.remove(i);
            }
        }
    }
    Ok(())
}

fn read_input(file_path: &str) -> Result<(Vec<Board>, Vec<i32>), MyError> {
    let mut lines = file_lines(file_path)?;

    let numbers: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    lines.next();

    let mut boards: Vec<Board> = vec![];
    let mut board = Board::new();
    let mut row = 0;
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            boards.push(board);
            row = 0;
            board = Board::new();
        } else {
            let row_items: Vec<i32> = line
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse().unwrap())
                .collect();
            board.add_row(row_items, row);
            row += 1;
        }
    }
    boards.push(board);
    Ok((boards, numbers))
}
