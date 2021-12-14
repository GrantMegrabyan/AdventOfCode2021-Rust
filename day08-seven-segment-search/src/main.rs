use std::{io::{BufReader, Read, BufRead}, fs::File, collections::HashSet};
use shared::{read_first_arg, MyError};

fn main() -> Result<(), MyError> {
    let file_path = read_first_arg()?;
    let reader = BufReader::new(File::open(file_path)?);
    
    let unique_digits = count_unque_digits(reader);
    println!("Unique difits: {}", unique_digits);

    Ok(())
}

fn count_unque_digits<R: Read>(reader: BufReader<R>) -> usize {
    let unique_length: HashSet<usize> = vec![2, 3, 4, 7].into_iter().collect();

    reader.lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let right = l.split_once(" | ").unwrap();
            String::from(right.1)
        })
        .map(|l| l.split(" ").filter(|code| unique_length.contains(&code.len())).count())
        .sum()
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
}