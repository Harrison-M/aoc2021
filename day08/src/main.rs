use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

const DIGITS: [usize; 10] = [
    0b1110111, 0b0100100, 0b1011101, 0b1101101, 0b0101110, 0b1101011, 0b1111011, 0b0100101,
    0b1111111, 0b1101111,
];

fn parse_lines(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| line.split(' ').filter(|&s| s != "|").collect())
        .collect()
}

fn part1(displays: &[Vec<&str>]) -> usize {
    displays
        .iter()
        .map(|display| {
            display[10..14]
                .iter()
                .map(|s| s.len())
                .filter(|&l| l == 2 || l == 3 || l == 4 || l == 7)
                .count()
        })
        .sum()
}

fn deduce(display: &[&str]) -> usize {
    // Invert this
    let all: HashSet<char> = ['a', 'b', 'c', 'd', 'e', 'f', 'g'].into();
    let mut possibilities: Vec<HashSet<char>> = (0..7).map(|_| all.clone()).collect();

    let digits: Vec<HashSet<char>> = (&display[0..10])
        .iter()
        .map(|d| d.chars().collect())
        .collect();

    let one = digits.iter().find(|d| d.len() == 2).expect("No 1 found");

    for (i, wire_ps) in possibilities.iter_mut().enumerate() {
        if i == 2 || i == 5 {
            *wire_ps = one.clone();
        } else {
            *wire_ps = wire_ps.difference(one).copied().collect();
        }
    }

    let seven = digits.iter().find(|d| d.len() == 3).expect("No 7 found");

    for (i, wire_ps) in possibilities.iter_mut().enumerate() {
        if i == 0 || i == 2 || i == 5 {
            *wire_ps = wire_ps.intersection(seven).copied().collect();
        } else {
            *wire_ps = wire_ps.difference(seven).copied().collect();
        }
    }

    // 0 should now be definitely identified

    let four = digits.iter().find(|d| d.len() == 4).expect("No 4 found");

    for (i, wire_ps) in possibilities.iter_mut().enumerate() {
        if i == 1 || i == 2 || i == 3 || i == 5 {
            *wire_ps = wire_ps.intersection(four).copied().collect();
        } else {
            *wire_ps = wire_ps.difference(four).copied().collect();
        }
    }

    // 6 is the 6-length output with 5 but not 2, which will allow us to identify 2 and 5
    let six = digits
        .iter()
        .find(|d| d.len() == 6 && d.intersection(&possibilities[2]).count() == 1)
        .expect("No 6 found");

    for (i, wire_ps) in possibilities.iter_mut().enumerate() {
        if i == 2 {
            *wire_ps = wire_ps.difference(six).copied().collect();
        } else {
            *wire_ps = wire_ps.intersection(six).copied().collect();
        }
    }

    // 0 is the 6-length output with 1 but not 3, which will allow us to identify 1 and 3
    let zero = digits
        .iter()
        .find(|d| d.len() == 6 && d.intersection(&possibilities[1]).count() == 1)
        .expect("No 0 found");

    for (i, wire_ps) in possibilities.iter_mut().enumerate() {
        if i == 3 {
            *wire_ps = wire_ps.difference(zero).copied().collect();
        } else {
            *wire_ps = wire_ps.intersection(zero).copied().collect();
        }
    }

    let mut answers: HashMap<char, u32> = HashMap::new();
    // We now have 0, 2, 3, and 5. The 5-length with all of those (3) gives us 6.
    answers.insert(*possibilities[0].iter().next().unwrap(), 0);
    answers.insert(*possibilities[2].iter().next().unwrap(), 2);
    answers.insert(*possibilities[3].iter().next().unwrap(), 3);
    answers.insert(*possibilities[5].iter().next().unwrap(), 5);
    let three_check_set: HashSet<char> = answers.keys().copied().collect();
    let three = digits
        .iter()
        .find(|d| d.len() == 5 && d.intersection(&three_check_set).count() == 4)
        .expect("No 3 found");
    answers.insert(*three.difference(&three_check_set).next().unwrap(), 6);

    // We can now use 9 to find 1
    let nine_check_set: HashSet<char> = answers.keys().copied().collect();
    let nine = digits
        .iter()
        .find(|d| d.len() == 6 && d.intersection(&nine_check_set).count() == 5)
        .expect("No 9 found");

    answers.insert(*nine.difference(&nine_check_set).next().unwrap(), 1);

    // 0, 1, 2, 3, 5, 6. The remaining digit is 4.
    answers.insert(
        *all.difference(&answers.keys().copied().collect())
            .next()
            .unwrap(),
        4,
    );

    let digit_lookup: HashMap<usize, usize> = DIGITS
        .iter()
        .copied()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect();

    (&display[10..14])
        .iter()
        .map(|d| {
            digit_lookup
                .get(
                    &d.chars()
                        .map(|c| 2usize.pow(*answers.get(&c).unwrap()))
                        .sum(),
                )
                .copied()
                .unwrap()
        })
        .reduce(|acc, d| acc * 10 + d)
        .unwrap()
}

fn part2(displays: &[Vec<&str>]) -> usize {
    displays.iter().map(|d| deduce(d)).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");

    let displays = parse_lines(&contents);

    println!("Part 1: {}", part1(&displays));
    println!("Part 2: {}", part2(&displays));
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = include_str!("sample");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_lines(SAMPLE)), 26);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_lines(SAMPLE)), 61229);
    }
}
