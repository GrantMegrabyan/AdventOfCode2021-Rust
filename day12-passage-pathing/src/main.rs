use shared::{read_first_arg, MyError};
use std::collections::{HashMap, VecDeque};
use std::fs;

fn main() -> Result<(), MyError> {
    let file_path = read_first_arg()?;
    let edges = string_to_edges(fs::read_to_string(file_path).unwrap());

    let all_paths_count = count_all_paths(&edges);
    println!("Number of all paths: {}", all_paths_count);

    Ok(())
}

fn count_all_paths(edges: &HashMap<String, Vec<String>>) -> usize {
    let mut visited = VecDeque::<&str>::new();
    visited.push_back("start");
    dfs(&edges, &mut visited, "start").len()
}

fn dfs<'a>(
    edges: &'a HashMap<String, Vec<String>>,
    visited: &mut VecDeque<&'a str>,
    start: &str,
) -> Vec<Vec<&'a str>> {
    if start == "end" {
        return vec![Vec::from_iter(visited.iter().map(|&a| a))];
    }

    if !edges.contains_key(start) {
        return vec![];
    }

    let next_caves = edges[start]
        .iter()
        .filter(|&adj| adj != &adj.to_lowercase() || !visited.contains(&adj.as_str()))
        .map(|s| s.as_str())
        .collect::<Vec<&str>>();

    let mut paths = vec![];
    for cave in next_caves {
        visited.push_back(cave);
        paths.append(&mut dfs(edges, visited, &cave));
        visited.pop_back();
    }
    paths
}

fn string_to_edges(s: String) -> HashMap<String, Vec<String>> {
    s.trim()
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .fold(HashMap::new(), |mut map, (from, to)| {
            let (from, to) = (from.trim(), to.trim());
            map.entry(from.to_string())
                .or_insert(vec![])
                .push(to.to_string());
            map.entry(to.to_string())
                .or_insert(vec![])
                .push(from.to_string());
            map
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_all_paths() {
        let input = r#"
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end"#
            .to_string();
        let edges: HashMap<String, Vec<String>> = string_to_edges(input);

        let expected = 10;
        let actual = count_all_paths(&edges);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_conut_all_paths_2() {
        let input = r#"
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc"#
            .to_string();
        let edges: HashMap<String, Vec<String>> = string_to_edges(input);

        println!("{:?}", edges);

        let expected = 19;
        let actual = count_all_paths(&edges);
        assert_eq!(expected, actual);
    }
}
