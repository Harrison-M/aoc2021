use std::{collections::HashMap, env, fs};

fn parse_input(input: &str) -> (&str, HashMap<&str, &str>) {
    let sections: Vec<_> = input.split("\n\n").collect();
    let rules: HashMap<_, _> = sections[1]
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();
    (sections[0], rules)
}

fn step(polymer: String, rules: &HashMap<&str, &str>) -> String {
    let chars: Vec<_> = polymer.chars().collect();
    let pairs: Vec<_> = chars
        .windows(2)
        .map(|pair| format!("{}{}", pair[0], pair[1]))
        .collect();

    let mut next = String::from("");

    for pair in pairs {
        let insert = rules.get(&pair[..]);
        let first = pair.chars().next().unwrap();
        next = match insert {
            Some(addition) => format!("{}{}{}", next, first, addition),
            None => format!("{}{}", next, first),
        }
    }

    format!("{}{}", next, chars.last().unwrap())
}

fn part1(init: &str, rules: &HashMap<&str, &str>) -> usize {
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

/*
fn part2(init: &str, rules: &HashMap<&str, &str>) -> usize {
    let mut polymer = init.to_string();

    for _ in 0..40 {
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
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");

    let (polymer, rules) = parse_input(&contents);

    println!("Part 1: {}", part1(polymer, &rules));
    // println!("Part 2: {}", part2(polymer, &rules));
}
