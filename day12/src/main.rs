use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

fn count_routes<'a>(cave: Cave<'a>, network: &'a Network, visited: &mut HashSet<&'a str>) -> usize {
    if cave == End {
        return 1;
    }

    if let Small(name) = cave {
        visited.insert(name);
    }

    let mut result = 0;
    for link in network.get(&cave).unwrap().iter() {
        match link {
            Small(name) if visited.contains(name) => continue,
            _ => (),
        }

        result += count_routes(*link, network, visited);
    }

    if let Small(name) = cave {
        visited.remove(name);
    }

    result
}

fn part1(network: &Network) -> usize {
    count_routes(Start, network, &mut HashSet::new())
}

fn count_part2_routes<'a>(
    cave: Cave<'a>,
    network: &'a Network,
    visited: &mut HashSet<&'a str>,
    visited_small_twice: bool,
) -> usize {
    if cave == End {
        return 1;
    }

    println!("{:?}", visited);

    let mut result = 0;
    if visited_small_twice {
        for link in network.get(&cave).unwrap().iter() {
            match link {
                Small(name) if !visited.insert(name) => continue,
                _ => (),
            }

            result += count_part2_routes(*link, network, visited, true);

            if let Small(name) = link {
                visited.remove(name);
            }
        }
    } else {
        for link in network.get(&cave).unwrap().iter() {
            let second_visit = if let Small(name) = link {
                !visited.insert(name)
            } else {
                false
            };

            result += count_part2_routes(*link, network, visited, second_visit);

            match (link, second_visit) {
                (Small(name), false) => visited.remove(name),
                _ => false,
            };
        }
    }

    result
}

fn part2(network: &Network) -> usize {
    count_part2_routes(Start, network, &mut HashSet::new(), false)
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
