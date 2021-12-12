use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Cave<'a> {
    Big(&'a str),
    Small(&'a str),
    Start,
    End,
}

use Cave::*;

type Network<'a> = HashMap<Cave<'a>, Vec<Cave<'a>>>;

fn parse_input(input: &str) -> Network {
    let mut network: Network = HashMap::new();
    for line in input.lines() {
        let cave_pair: Vec<_> = line
            .split('-')
            .map(|cave| match cave {
                "start" => Start,
                "end" => End,
                _ => {
                    if cave.chars().next().unwrap().is_lowercase() {
                        Small(cave)
                    } else {
                        Big(cave)
                    }
                }
            })
            .collect();

        let (cave1, cave2) = (cave_pair[0], cave_pair[1]);
        if cave1 != End && cave2 != Start {
            network
                .entry(cave1)
                .and_modify(|exits| exits.push(cave2))
                .or_insert_with(|| vec![cave2]);
        }
        if cave2 != End && cave1 != Start {
            network
                .entry(cave2)
                .and_modify(|exits| exits.push(cave1))
                .or_insert_with(|| vec![cave1]);
        }
    }

    network
}

fn count_routes<'a>(cave: Cave<'a>, network: &'a Network, visited: &HashSet<&'a str>) -> usize {
    if cave == End {
        return 1;
    }
    let mut next_visited = visited.clone();
    if let Small(name) = cave {
        next_visited.insert(name);
    }
    network
        .get(&cave)
        .unwrap()
        .iter()
        .filter(|link| match link {
            Small(name) => !visited.contains(name),
            _ => true,
        })
        .map(|next| count_routes(*next, network, &next_visited))
        .sum()
}

fn part1(network: &Network) -> usize {
    count_routes(Start, network, &HashSet::new())
}

fn count_part2_routes<'a>(
    cave: Cave<'a>,
    network: &'a Network,
    visited: &HashSet<&'a str>,
    visited_small_twice: bool,
) -> usize {
    if cave == End {
        return 1;
    }
    let mut next_visited = visited.clone();
    if let Small(name) = cave {
        next_visited.insert(name);
    }
    if visited_small_twice {
        network
            .get(&cave)
            .unwrap()
            .iter()
            .filter(|link| match link {
                Small(name) => !visited.contains(name),
                _ => true,
            })
            .map(|next| count_part2_routes(*next, network, &next_visited, true))
            .sum()
    } else {
        network
            .get(&cave)
            .unwrap()
            .iter()
            .map(|link| match link {
                Small(name) if visited.contains(name) => (link, true),
                _ => (link, false),
            })
            .map(|(next, second_visit)| {
                count_part2_routes(*next, network, &next_visited, second_visit)
            })
            .sum()
    }
}

fn part2(network: &Network) -> usize {
    count_part2_routes(Start, network, &HashSet::new(), false)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");

    let network = parse_input(&contents);

    println!("Part 1: {}", part1(&network));
    println!("Part 2: {}", part2(&network));
}

#[cfg(test)]
mod test {
    use super::*;
    const SAMPLES: [&str; 3] = [
        include_str!("sample1"),
        include_str!("sample2"),
        include_str!("sample3"),
    ];

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&parse_input(SAMPLES[0])), 10);
        assert_eq!(part1(&parse_input(SAMPLES[1])), 19);
        assert_eq!(part1(&parse_input(SAMPLES[2])), 226);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2(&parse_input(SAMPLES[0])), 36);
        assert_eq!(part2(&parse_input(SAMPLES[1])), 103);
        assert_eq!(part2(&parse_input(SAMPLES[2])), 3509);
    }
}
