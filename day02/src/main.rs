use regex::Regex;
use std::{env, fs};

const INSTRUCTION_RE_STR: &str = r"(\w+) (\d+)";

enum Command {
    Forward,
    Up,
    Down,
}

struct Instruction {
    command: Command,
    value: usize,
}

fn parse_instructions(instruction_text: &str) -> Result<Vec<Instruction>, regex::Error> {
    let instruction_re = Regex::new(INSTRUCTION_RE_STR)?;

    Ok(instruction_text
        .lines()
        .map(|line| {
            let caps = instruction_re.captures(line).expect("Invalid instruction");
            let command = match caps.get(1).map(|c| c.as_str()) {
                Some("forward") => Command::Forward,
                Some("down") => Command::Down,
                Some("up") => Command::Up,
                _ => panic!("Invalid instruction"),
            };
            let value = caps
                .get(2)
                .map(|val| val.as_str().parse().expect("Invalid value"))
                .expect("Missing value");
            Instruction { command, value }
        })
        .collect())
}

fn part1(instructions: &Vec<Instruction>) -> (usize, usize) {
    instructions
        .iter()
        .fold((0, 0), |(h, d), instruction| match instruction.command {
            Command::Forward => (h + instruction.value, d),
            Command::Up => (h, d - instruction.value),
            Command::Down => (h, d + instruction.value),
        })
}

fn part2(instructions: &Vec<Instruction>) -> (usize, usize, usize) {
    instructions
        .iter()
        .fold((0, 0, 0), |(h, d, a), instruction| {
            match instruction.command {
                Command::Forward => (h + instruction.value, d + a * instruction.value, a),
                Command::Up => (h, d, a - instruction.value),
                Command::Down => (h, d, a + instruction.value),
            }
        })
}

fn main() -> Result<(), regex::Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");

    let instructions = parse_instructions(&contents)?;

    let (x, y) = part1(&instructions);
    println!("Part 1: {}", x * y);

    let (x2, y2, _) = part2(&instructions);
    println!("Part 2: {}", x2 * y2);

    Ok(())
}

#[cfg(test)]
mod tests {}
