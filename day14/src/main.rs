use std::{collections::HashMap, env, fs};

type Cache = HashMap<((char, char), u8), HashMap<char, usize>>;

fn parse_input(input: &str) -> (&str, HashMap<(char, char), char>) {
    let sections: Vec<_> = input.split("\n\n").collect();
    let rules: HashMap<_, _> = sections[1]
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let mut pair = parts.next().unwrap().chars();
            (
                (pair.next().unwrap(), pair.next().unwrap()),
                parts.next().unwrap().chars().next().unwrap(),
            )
        })
        .collect();
    (sections[0], rules)
}

fn merge(target: &mut HashMap<char, usize>, source: &HashMap<char, usize>) {
    for (c, count) in source.iter() {
        target
            .entry(*c)
            .and_modify(|n| *n += *count)
            .or_insert(*count);
    }
}

fn added_characters(
    pair: (char, char),
    steps: u8,
    rules: &HashMap<(char, char), char>,
    cache: &mut Cache,
) -> HashMap<char, usize> {
    if let Some(result) = cache.get(&(pair, steps)) {
        return result.clone();
    }
    let next = steps - 1;
    let mut counts: HashMap<char, usize> = HashMap::new();
    let maybe_insert = rules.get(&pair);
    if let Some(insert) = maybe_insert {
        counts.insert(*insert, 1);
        if next != 0 {
            merge(
                &mut counts,
                &added_characters((pair.0, *insert), next, rules, cache),
            );
            merge(
                &mut counts,
                &added_characters((*insert, pair.1), next, rules, cache),
            );
        }
    }
    cache.insert((pair, steps), counts.clone());
    counts
}

fn step(polymer: String, rules: &HashMap<(char, char), char>) -> String {
    let chars: Vec<_> = polymer.chars().collect();
    let pairs: Vec<_> = chars.windows(2).map(|pair| (pair[0], pair[1])).collect();

    let mut next = String::from("");

    for pair in pairs {
        let insert = rules.get(&pair);
        let first = pair.0;
        next = match insert {
            Some(addition) => format!("{}{}{}", next, first, addition),
            None => format!("{}{}", next, first),
        }
    }

    format!("{}{}", next, chars.last().unwrap())
}

fn part1(init: &str, rules: &HashMap<(char, char), char>) -> usize {
    let mut polymer = init.to_string();

    for _ in 0..10 {
        polymer = step(polymer, rules);
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for element in polymer.chars() {
        counts.entry(element).and_modify(|c| *c += 1).or_insert(1);
    }

    let min = counts.values().min().unwrap();
    let max = counts.values().max().unwrap();

    max - min
}

fn part2(polymer: &str, rules: &HashMap<(char, char), char>) -> usize {
    let chars: Vec<char> = polymer.chars().collect();
    let mut counts = HashMap::new();

    // Initial counts
    for c in chars.iter() {
        counts.entry(*c).and_modify(|n| *n += 1).or_insert(1);
    }

    let pairs: Vec<_> = chars.windows(2).map(|pair| (pair[0], pair[1])).collect();
    let mut cache: Cache = HashMap::new();
    for pair in pairs {
        merge(&mut counts, &added_characters(pair, 40, rules, &mut cache))
    }

    let min = counts.values().min().unwrap();
    let max = counts.values().max().unwrap();

    max - min
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");

    let (polymer, rules) = parse_input(&contents);

    println!("Part 1: {}", part1(polymer, &rules));
    println!("Part 2: {}", part2(polymer, &rules));
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = include_str!("sample");

    #[test]
    fn part1_example() {
        let (polymer, rules) = parse_input(SAMPLE);
        assert_eq!(part1(polymer, &rules), 1588);
    }

    #[test]
    fn part2_example() {
        let (polymer, rules) = parse_input(SAMPLE);
        assert_eq!(part2(polymer, &rules), 2188189693529);
    }
}
