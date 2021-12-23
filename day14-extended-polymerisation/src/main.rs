use shared::{read_first_arg, MyError};
use std::collections::HashMap;

fn main() -> Result<(), MyError> {
    let file_path = read_first_arg()?;
    let input = std::fs::read_to_string(file_path).unwrap();
    let (template, rules) = read_input(&input);

    let mut poly = Polymerisation::new(template, rules);
    for _ in 0..40 {
        poly.step();
    }
    let (most, least) = poly.most_least_count();

    println!("Most comment - Least common = {}", most - least);

    Ok(())
}

struct Polymerisation {
    rules: HashMap<String, char>,
    pairs: HashMap<String, u64>,
    elements: HashMap<char, u64>,
}

impl Polymerisation {
    pub fn new(template: String, rules: HashMap<String, char>) -> Self {
        let mut pairs: HashMap<String, u64> = HashMap::new();
        let mut elements: HashMap<char, u64> = HashMap::new();

        let mut prev_char: Option<char> = None;
        for ch in template.chars() {
            *elements.entry(ch).or_insert(0) += 1;
            if let Some(prev_ch) = prev_char {
                let pair = format!("{}{}", prev_ch, ch);
                *pairs.entry(pair).or_insert(0) += 1;
            }
            prev_char = Some(ch);
        }

        Polymerisation {
            rules,
            pairs,
            elements,
        }
    }

    pub fn step(&mut self) {
        let mut new_pairs: HashMap<String, u64> = HashMap::new();
        for (pair, &count) in &self.pairs {
            let new_element = self.rules[pair];
            *self.elements.entry(new_element).or_insert(0) += count;

            let new_pair1 = format!("{}{}", &pair[0..1], new_element);
            *new_pairs.entry(new_pair1).or_insert(0) += count;

            let new_pair2 = format!("{}{}", new_element, &pair[1..2]);
            *new_pairs.entry(new_pair2).or_insert(0) += count;
        }
        self.pairs = new_pairs;
    }

    pub fn most_least_count(&self) -> (u64, u64) {
        let mut most = u64::MIN;
        let mut least = u64::MAX;

        for &count in self.elements.values() {
            most = count.max(most);
            least = count.min(least);
        }

        (most, least)
    }
}

fn read_input(input: &str) -> (String, HashMap<String, char>) {
    let (template, rules) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .filter_map(|l| {
            if l.trim().is_empty() {
                None
            } else {
                Some(l.trim())
            }
        })
        .map(|l| (l[0..2].to_string(), l[6..7].chars().last().unwrap()))
        .collect::<HashMap<_, _>>();

    (template.to_string(), rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_and_least_common_elements() {
        let input = r#"NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
        "#;
        let (template, rules) = read_input(input);

        let mut poly = Polymerisation::new(template.to_string(), rules);
        for _ in 0..10 {
            poly.step();
        }

        let (most, least) = poly.most_least_count();

        assert_eq!(1588, most - least);
    }
}
