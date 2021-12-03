use std::{env, fs};

fn one_counts(numbers: &Vec<&str>) -> Vec<usize> {
    let mut ones: Vec<usize> = vec![];
    for _ in numbers[0].chars() {
        ones.push(0);
    }
    for number in numbers.iter() {
        for (idx, digit) in number.chars().enumerate() {
            let count = ones.get(idx).unwrap_or(&0);
            ones[idx] = if digit == '1' { *count + 1 } else { *count }
        }
    }

    ones
}

fn part1(numbers: &Vec<&str>) -> usize {
    let threshold = numbers.len() / 2;
    let ones: Vec<usize> = one_counts(numbers);

    let (gb, eb) = ones
        .iter()
        .fold((String::new(), String::new()), |(gs, es), &count| {
            if count > threshold {
                (gs + "1", es + "0")
            } else {
                (gs + "0", es + "1")
            }
        });

    let gamma = usize::from_str_radix(&gb, 2).expect("Invalid gamma binary");
    let epsilon = usize::from_str_radix(&eb, 2).expect("Invalid epsilon binary");

    println!("{}, {}", gamma, epsilon);

    gamma * epsilon
}

fn part2(numbers: &Vec<&str>) -> usize {
    let mut oxygen_candidates = numbers.clone();
    let mut oxygen_check_bit: usize = 0;
    let oxygen_str = loop {
        if oxygen_candidates.len() == 1 {
            break oxygen_candidates.get(0).unwrap();
        }
        let ones: usize = oxygen_candidates.iter().fold(0, |acc, num| {
            if num
                .chars()
                .nth(oxygen_check_bit)
                .expect("Oxygen check bit past length")
                == '1'
            {
                acc + 1
            } else {
                acc
            }
        });
        let threshold: f32 = oxygen_candidates.len() as f32 / 2.0;
        let mcb = if ones as f32 >= threshold { '1' } else { '0' };
        oxygen_candidates = oxygen_candidates
            .into_iter()
            .filter(|num| num.chars().nth(oxygen_check_bit).unwrap() == mcb)
            .collect();
        oxygen_check_bit += 1;
    };

    let mut co2_candidates = numbers.clone();
    let mut co2_check_bit: usize = 0;
    let co2_str = loop {
        if co2_candidates.len() == 1 {
            break co2_candidates.get(0).unwrap();
        }
        let ones: usize = co2_candidates.iter().fold(0, |acc, num| {
            if num
                .chars()
                .nth(co2_check_bit)
                .expect("Oxygen check bit past length")
                == '1'
            {
                acc + 1
            } else {
                acc
            }
        });
        let threshold: f32 = co2_candidates.len() as f32 / 2.0;
        let lcb = if (ones as f32) < threshold { '1' } else { '0' };
        co2_candidates = co2_candidates
            .into_iter()
            .filter(|num| num.chars().nth(co2_check_bit).unwrap() == lcb)
            .collect();
        co2_check_bit += 1;
    };

    let oxygen = usize::from_str_radix(oxygen_str, 2).expect("Invalid oxygen binary");
    let co2 = usize::from_str_radix(co2_str, 2).expect("Invalid co2 binary");

    println!("{}, {}", oxygen, co2);

    oxygen * co2
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let lines: Vec<_> = contents.lines().collect();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("sample");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&SAMPLE.lines().collect()), 198);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&SAMPLE.lines().collect()), 230);
    }
}
