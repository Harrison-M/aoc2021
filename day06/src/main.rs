use std::{collections::HashMap, env, fs};

fn simulate(fish: &[usize], iterations: usize) -> usize {
    let mut frequencies: HashMap<usize, usize> = HashMap::new();

    for f in fish {
        frequencies.entry(*f).and_modify(|c| *c += 1).or_insert(1);
    }

    for _ in 0..iterations {
        let mut new_frequencies: HashMap<usize, usize> = HashMap::new();
        for (time, count) in frequencies.into_iter() {
            if time == 0 {
                new_frequencies
                    .entry(6)
                    .and_modify(|c| *c += count)
                    .or_insert(count);

                new_frequencies.insert(8, count);
            } else {
                new_frequencies
                    .entry(time - 1)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            }
        }
        frequencies = new_frequencies
    }

    frequencies.values().sum()
}

fn part1(fish: &[usize]) -> usize {
    simulate(fish, 80)
}

fn part2(fish: &[usize]) -> usize {
    simulate(fish, 256)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let fish: Vec<usize> = contents
        .split(',')
        .map(|n| n.trim().parse().expect("Invalid input"))
        .collect();

    println!("Part 1: {}", part1(&fish));
    println!("Part 1: {}", part2(&fish));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: [usize; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn part1_example() {
        assert_eq!(part1(&SAMPLE), 5934);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&SAMPLE), 26984457539);
    }
}
