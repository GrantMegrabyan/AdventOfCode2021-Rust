#[macro_use]
extern crate lazy_static;

use shared::{read_first_arg, MyError};
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Read},
};

lazy_static! {
    static ref CLOSE_TO_OPEN: HashMap<char, char> =
        HashMap::<_, _>::from_iter([(')', '('), (']', '['), ('}', '{'), ('>', '<'),]);
    static ref OPEN_TO_CLOSE: HashMap<char, char> =
        HashMap::<_, _>::from_iter([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>'),]);
    static ref CORRUPT_SCORES: HashMap<char, u32> =
        HashMap::<_, _>::from_iter([(')', 3), (']', 57), ('}', 1197), ('>', 25137),]);
    static ref INCOMPLETE_SCORES: HashMap<char, u64> =
        HashMap::<_, _>::from_iter([(')', 1), (']', 2), ('}', 3), ('>', 4),]);
}

fn main() -> Result<(), MyError> {
    let file_path = read_first_arg()?;
    let reader = BufReader::new(File::open(file_path.clone())?);

    let total_score = calculate_total_syntax_error_score(reader);
    println!("Total syntax error score: {}", total_score);

    let reader = BufReader::new(File::open(file_path.clone())?);
    let incomplete_middle_score = calculate_incomplete_middle_score(reader);
    println!("Incomplete middle score: {}", incomplete_middle_score);

    Ok(())
}

fn calculate_total_syntax_error_score<R: Read>(reader: BufReader<R>) -> u32 {
    reader
        .lines()
        .filter_map(|line| get_first_illegal_char(&line.unwrap()))
        .map(|ch| CORRUPT_SCORES[&ch])
        .sum()
}

fn get_first_illegal_char(line: &str) -> Option<char> {
    let mut stack = VecDeque::new();
    for ch in line.chars() {
        if CLOSE_TO_OPEN.contains_key(&ch) {
            match stack.pop_front() {
                Some(prev) if prev != CLOSE_TO_OPEN[&ch] => return Some(ch),
                None => return Some(ch),
                _ => (),
            }
        } else {
            stack.push_front(ch);
        }
    }
    None
}

fn calculate_incomplete_middle_score<R: Read>(reader: BufReader<R>) -> u64 {
    let mut scores = reader
        .lines()
        .filter_map(|line| get_completion_string(&line.unwrap()))
        .map(|s| s.chars().fold(0, |acc, ch| acc * 5 + INCOMPLETE_SCORES[&ch]))
        .collect::<Vec<_>>();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn get_completion_string(line: &str) -> Option<String> {
    let mut stack = VecDeque::new();
    for ch in line.chars() {
        if CLOSE_TO_OPEN.contains_key(&ch) {
            match stack.pop_front() {
                Some(prev) if prev != CLOSE_TO_OPEN[&ch] => return None,
                None => return None,
                _ => (),
            }
        } else {
            stack.push_front(ch);
        }
    }

    if stack.is_empty() {
        return None;
    }

    Some(stack.iter().map(|ch| OPEN_TO_CLOSE[ch]).collect())
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

    #[test]
    fn test_get_completion_string() {
        let expected = Some(String::from("}}]])})]"));
        let actual = get_completion_string("[({(<(())[]>[[{[]{<()<>>");
        assert_eq!(expected, actual);

        let expected = Some(String::from(")}>]})"));
        let actual = get_completion_string("[(()[<>])]({[<{<<[]>>(");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_incomplete_middle_score() {
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

        let expected = 288957;
        let actual = calculate_incomplete_middle_score(reader);
        assert_eq!(expected, actual);
    }
}
