#[macro_use]
extern crate lazy_static;

use shared::{read_first_arg, MyError};
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Read},
};

lazy_static! {
    static ref BRACKETS: HashMap<char, char> =
        HashMap::<_, _>::from_iter([(')', '('), (']', '['), ('}', '{'), ('>', '<'),]);
    static ref SCORES: HashMap<char, u32> =
        HashMap::<_, _>::from_iter([(')', 3), (']', 57), ('}', 1197), ('>', 25137),]);
}

fn main() -> Result<(), MyError> {
    let file_path = read_first_arg()?;
    let reader = BufReader::new(File::open(file_path.clone())?);

    let total_score = calculate_total_syntax_error_score(reader);
    println!("Total syntax error score: {}", total_score);

    Ok(())
}

fn calculate_total_syntax_error_score<R: Read>(reader: BufReader<R>) -> u32 {
    reader
        .lines()
        .filter_map(|line| get_first_illegal_char(&line.unwrap()))
        .map(|ch| SCORES[&ch])
        .sum()
}

fn get_first_illegal_char(line: &str) -> Option<char> {
    let mut stack = VecDeque::new();
    for ch in line.chars() {
        if BRACKETS.contains_key(&ch) {
            match stack.pop_front() {
                Some(prev) if prev != BRACKETS[&ch] => return Some(ch),
                None => return Some(ch),
                _ => (),
            }
        } else {
            stack.push_front(ch);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_total_syntax_error_score() {
        let input = r#"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
        "#;
        let cursor = std::io::Cursor::new(input.trim());
        let reader = BufReader::new(cursor);

        let expected = 26397;
        let actual = calculate_total_syntax_error_score(reader);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_first_illegal_char() {
        let expected = None;
        let actual = get_first_illegal_char("[({(<(())[]>[[{[]{<()<>>");
        assert_eq!(expected, actual);

        let expected = Some('}');
        let actual = get_first_illegal_char("{([(<{}[<>[]}>{[]{[(<()>");
        assert_eq!(expected, actual);

        let expected = Some(')');
        let actual = get_first_illegal_char("[[<[([]))<([[{}[[()]]]");
        assert_eq!(expected, actual);
    }
}
