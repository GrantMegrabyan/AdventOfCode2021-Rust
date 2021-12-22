use shared::{read_first_arg, MyError};
use std::collections::HashMap;

fn main() -> Result<(), MyError> {
    let file_path = read_first_arg()?;
    let input = std::fs::read_to_string(file_path).unwrap();
    let (template, rules) = read_input(&input);

    let mut poly = Polymerisation::new(template.to_string(), rules);
    for _ in 0..10 {
        poly.step();
    }
    let (most, least) = poly.most_least_count();

    println!("Most comment - Least common = {}", most - least);

    Ok(())
}

struct Polymerisation {
    pub template: String,
    rules: HashMap<String, char>,
    counts: HashMap<char, usize>,
    step_counter: usize,
}

impl Polymerisation {
    pub fn new(template: String, rules: HashMap<String, char>) -> Self {
        let mut counts = HashMap::new();
        template
            .chars()
            .for_each(|ch| *counts.entry(ch).or_insert(0) += 1 as usize);

        Polymerisation {
            template,
            rules,
            counts,
            step_counter: 0,
        }
    }

    pub fn step(&mut self) {
        self.step_counter += 1;

        let mut new_template = String::new();
        for i in 0..self.template.len() - 1 {
            let pair = &self.template[i..i + 2];
            if self.rules.contains_key(pair) {
                let new_element = self.rules[pair];
                new_template += &format!("{}{}", &pair[0..1], new_element);
                *self.counts.entry(new_element).or_insert(0) += 1 as usize;
            } else {
                new_template += &pair[0..1];
            }
        }
        new_template += &self.template[self.template.len() - 1..];
        self.template = new_template;
    }

    pub fn most_least_count(&self) -> (usize, usize) {
        let mut most = usize::MIN;
        let mut least = usize::MAX;

        for (_, &count) in &self.counts {
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
            println!("Step {}: {}", poly.step_counter, poly.template);
        }

        let (most, least) = poly.most_least_count();

        assert_eq!(1588, most - least);
    }
}
