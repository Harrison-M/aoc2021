use std::{collections::HashSet, env, fs};

enum Fold {
    X(u16),
    Y(u16),
}

struct Paper {
    dots: HashSet<(u16, u16)>,
    fold_stack: Vec<Fold>,
}

fn parse_input(input: &str) {
    let mut parts: Vec<_> = input.split("\n\n").collect();

    let dots: HashSet<(u16, u16)> = parts[0]
        .lines()
        .map(|line| {
            let mut points = line.split(',');
            (
                points.next().unwrap().parse().unwrap(),
                points.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let fold_stack = parts[1].lines()
        .map(
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
}
