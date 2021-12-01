use std::env;
use std::fs;

fn parse_numbers<'a>(num_strs: impl Iterator<Item = &'a str>) -> Vec<u32> {
    num_strs
        .map(|line| line.parse::<u32>().expect("Failed to parse number"))
        .collect()
}

fn part1(numbers: &Vec<u32>) -> usize {
    numbers
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

fn part2(numbers: &Vec<u32>) -> usize {
    let sums: Vec<u32> = numbers
        .windows(3)
        .map(|window| window[0] + window[1] + window[2])
        .collect();

    sums.windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");

    let numbers = parse_numbers(contents.lines());
    println!("{}", part1(&numbers));
    println!("{}", part2(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let sample = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(part1(&sample), 7);
    }

    #[test]
    fn part2_example() {
        let sample = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(part2(&sample), 5);
    }
}
