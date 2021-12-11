use std::{env, fs};

enum ScoreType {
    Corrupted(usize),
    Incomplete(usize),
}

use ScoreType::*;

fn chunk_match(begin: char, end: char) -> bool {
    begin == '(' && end == ')'
        || begin == '[' && end == ']'
        || begin == '{' && end == '}'
        || begin == '<' && end == '>'
}

fn score_line(line: &str) -> ScoreType {
    let mut stack: Vec<char> = Vec::new();
    let mut iter = line.chars();
    loop {
        if let Some(cur) = iter.next() {
            match cur {
                '(' | '[' | '{' | '<' => stack.push(cur),
                ')' | ']' | '}' | '>' => {
                    let opening = stack.pop();
                    match opening {
                        Some(o) if chunk_match(o, cur) => (),
                        _ => {
                            break Corrupted(match cur {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                _ => panic!("Invalid close found in scoring"),
                            })
                        }
                    }
                }
                _ => panic!("Invalid input"),
            }
        } else {
            break Incomplete(stack.iter().rev().fold(0, |acc, opening| {
                acc * 5
                    + match opening {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("Invalid opening found in scoring"),
                    }
            }));
        }
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut incompletes: Vec<usize> = Vec::new();

    // avoiding side effects is for nerds
    let part1 = input
        .lines()
        .map(score_line)
        .fold(0, |acc, score| match score {
            Corrupted(s) => acc + s,
            Incomplete(s) => {
                incompletes.push(s);
                acc
            }
        });

    incompletes.sort_unstable();

    (part1, incompletes[incompletes.len() / 2])
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");

    let (part1, part2) = solve(&contents);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = include_str!("sample");

    #[test]
    fn example() {
        assert_eq!(solve(SAMPLE), (26397, 288957));
    }
}
