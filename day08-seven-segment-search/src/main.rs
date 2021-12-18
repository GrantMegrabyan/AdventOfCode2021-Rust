use shared::{read_first_arg, MyError};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn main() -> Result<(), MyError> {
    let file_path = read_first_arg()?;
    let reader = BufReader::new(File::open(file_path.clone())?);

    let unique_digits = count_unque_digits(reader);
    println!("Unique digits: {}", unique_digits);

    let reader = BufReader::new(File::open(file_path)?);
    let decoded = decode_signal(reader);
    println!("Decoded signal: {}", decoded);

    Ok(())
}

fn count_unque_digits<R: Read>(reader: BufReader<R>) -> usize {
    let unique_length: HashSet<usize> = vec![2, 3, 4, 7].into_iter().collect();

    reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let right = l.split_once(" | ").unwrap();
            String::from(right.1)
        })
        .map(|l| {
            l.split(" ")
                .filter(|code| unique_length.contains(&code.len()))
                .count()
        })
        .sum()
}

fn decode_signal<R: Read>(reader: BufReader<R>) -> i32 {
    let mut letters: HashMap<char, i32> = HashMap::with_capacity(7);

    let mut segments: HashMap<i32, i32> = HashMap::with_capacity(7);
    for i in 0..7 {
        segments.insert(i, 1 << (6 - i));
    }

    let mut digits: HashMap<i32, i32> = HashMap::with_capacity(10);
    digits.insert(0, get_number(&segments, vec![0, 1, 2, 4, 5, 6]));
    digits.insert(1, get_number(&segments, vec![2, 5]));
    digits.insert(2, get_number(&segments, vec![0, 2, 3, 4, 6]));
    digits.insert(3, get_number(&segments, vec![0, 2, 3, 5, 6]));
    digits.insert(4, get_number(&segments, vec![1, 2, 3, 5]));
    digits.insert(5, get_number(&segments, vec![0, 1, 3, 5, 6]));
    digits.insert(6, get_number(&segments, vec![0, 1, 3, 4, 5, 6]));
    digits.insert(7, get_number(&segments, vec![0, 2, 5]));
    digits.insert(8, get_number(&segments, vec![0, 1, 2, 3, 4, 5, 6]));
    digits.insert(9, get_number(&segments, vec![0, 1, 2, 3, 5, 6]));

    let pattern_to_digit: HashMap<i32, i32> = digits.iter()
        .map(|(digit, pattern)| (*pattern, *digit))
        .collect();

    let mut result = 0;

    for line in reader.lines() {
        for ch in "abcdefg".chars() {
            letters.insert(ch, 0b1111111);
        }

        let line = line.unwrap();
        let (left, right) = line.split_once(" | ").unwrap();
        let mut codes = left.split(" ").collect::<Vec<_>>();
        codes.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());

        let mut seven_code = "";
        for code in &codes {
            let pattern = if code.len() == 2 {
                digits[&1]
            } else if code.len() == 3 {
                seven_code = code;
                digits[&7]
            } else if code.len() == 4 {
                digits[&4]
            } else if code.len() == 5 {
                if seven_code.chars().all(|ch| code.contains(ch)) {
                    digits[&3]
                } else {
                    0
                }
            } else if code.len() == 6 {
                if seven_code.chars().all(|ch| code.contains(ch)) {
                    0
                } else {
                    digits[&6]
                }
            } else {
                0
            };

            if pattern == 0 {
                continue;
            }

            for ch in "abcdefg".chars() {
                if code.contains(ch) {
                    letters.insert(ch, letters[&ch] & pattern);
                } else {
                    letters.insert(ch, letters[&ch] & invert(pattern));
                }
            }
        }

        let mut order = 1000;
        result += right
            .split(" ")
            .map(|code| code.chars().fold(0, |acc, ch| acc | letters[&ch]))
            .map(|pattern| {
                let num = pattern_to_digit[&pattern] * order;
                order = order / 10;
                num
            })
            .sum::<i32>();
    }

    result
}

fn invert(num: i32) -> i32 {
    let mut inverted = num;
    for i in 0..7 {
        inverted = inverted ^ (1 << i);
    }
    inverted
}

fn get_number(segments: &HashMap<i32, i32>, num_segments: Vec<i32>) -> i32 {
    num_segments.iter().fold(0, |acc, seg| acc | segments[seg])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_unique_digits() {
        let input = r#"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        "#;
        let cursor = std::io::Cursor::new(input.trim());
        let reader = BufReader::new(cursor);

        let unique_digits = count_unque_digits(reader);

        assert_eq!(26, unique_digits)
    }

    #[test]
    fn test_decode_digits() {
        let input = r#"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        "#;
        let cursor = std::io::Cursor::new(input.trim());
        let reader = BufReader::new(cursor);

        let unique_digits = decode_signal(reader);

        assert_eq!(61229, unique_digits)
    }
}
