use std::{env, fs};

fn part1(crabs: &[isize]) -> isize {
    let mut x: isize = 0;
    let mut maybe_prev: Option<isize> = None;

    loop {
        let next: isize = crabs.iter().map(|crab| (crab - x).abs()).sum();

        if let Some(prev) = maybe_prev {
            if next > prev {
                break prev;
            }
        }

        maybe_prev.replace(next);
        x += 1;
    }
}

// Could have used Gauss' formula here but I forgot about it
fn triangle_sum(n: isize) -> isize {
    (1..=n).sum()
}

fn part2(crabs: &[isize]) -> isize {
    let mut x: isize = 0;
    let mut maybe_prev: Option<isize> = None;

    loop {
        let next: isize = crabs
            .iter()
            .map(|crab| triangle_sum((crab - x).abs()))
            .sum();

        if let Some(prev) = maybe_prev {
            if next > prev {
                break prev;
            }
        }

        maybe_prev.replace(next);
        x += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");

    let crabs: Vec<isize> = contents
        .split(',')
        .map(|n| n.trim().parse().expect("Invalid input"))
        .collect();

    println!("Part 1: {}", part1(&crabs));
    println!("Part 2: {}", part2(&crabs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: [isize; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn part1_example() {
        assert_eq!(part1(&SAMPLE), 37);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&SAMPLE), 168);
    }
}
