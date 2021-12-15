use std::{
    collections::HashSet,
    env,
    fmt::{Error, Write},
    fs,
};

#[derive(Clone, Copy)]
enum Fold {
    X(u16),
    Y(u16),
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Dot(u16, u16);

#[derive(Clone)]
struct Paper {
    dots: HashSet<Dot>,
    fold_stack: Vec<Fold>,
}

impl Paper {
    fn fold(&mut self) -> bool {
        let maybe_fold_line = self.fold_stack.pop();
        if let Some(fold_line) = maybe_fold_line {
            let moved_dots: Vec<_> = match fold_line {
                Fold::X(fx) => self
                    .dots
                    .iter()
                    .filter(|Dot(x, _)| x > &fx)
                    .copied()
                    .collect(),
                Fold::Y(fy) => self
                    .dots
                    .iter()
                    .filter(|Dot(_, y)| y > &fy)
                    .copied()
                    .collect(),
            };

            for dot in moved_dots {
                let new_dot = match (fold_line, dot) {
                    (Fold::X(fx), Dot(x, y)) => Dot(x - 2 * (x - fx), y),
                    (Fold::Y(fy), Dot(x, y)) => Dot(x, y - 2 * (y - fy)),
                };
                self.dots.remove(&dot);
                self.dots.insert(new_dot);
            }

            true
        } else {
            false
        }
    }
}

fn parse_input(input: &str) -> Paper {
    let parts: Vec<_> = input.split("\n\n").collect();

    let dots: HashSet<Dot> = parts[0]
        .lines()
        .map(|line| {
            let mut points = line.split(',');
            Dot(
                points.next().unwrap().parse().unwrap(),
                points.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let fold_stack: Vec<_> = parts[1]
        .lines()
        .rev()
        .map(|line| {
            let instructions: Vec<_> = line.split('=').collect();
            let num: u16 = instructions[1].parse().unwrap();
            if instructions[0] == "fold along x" {
                Fold::X(num)
            } else {
                Fold::Y(num)
            }
        })
        .collect();

    Paper { dots, fold_stack }
}

fn part1(init: &Paper) -> usize {
    let mut paper = init.clone();

    paper.fold();
    paper.dots.len()
}

fn part2(mut paper: Paper) -> Result<String, Error> {
    loop {
        if !paper.fold() {
            break;
        }
    }

    let max_x = paper.dots.iter().map(|Dot(x, _)| x).max().unwrap();
    let max_y = paper.dots.iter().map(|Dot(_, y)| y).max().unwrap();

    let mut output: String = String::new();

    for y in 0..=*max_y {
        for x in 0..=*max_x {
            if paper.dots.contains(&Dot(x, y)) {
                output.write_char('#')?;
            } else {
                output.write_char('.')?;
            }
        }
        output.write_char('\n')?;
    }

    Ok(output)
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");

    let paper = parse_input(&contents);

    println!("Part 1: {}", part1(&paper));
    println!("Part 2:");
    print!("{}", part2(paper)?);

    Ok(())
}
