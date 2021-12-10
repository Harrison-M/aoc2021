use std::{env, fs};

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("Non-digit character found"))
                .collect()
        })
        .collect()
}

fn part1(grid: &[Vec<u32>]) -> u32 {
    let mut risk: u32 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let mut neighbors: Vec<u32> = vec![];
            if x != 0 {
                neighbors.push(grid[y][x - 1])
            }
            if y != 0 {
                neighbors.push(grid[y - 1][x])
            }
            if let Some(east) = grid[y].get(x + 1) {
                neighbors.push(*east);
            }
            if let Some(south) = grid.get(y + 1) {
                neighbors.push(south[x]);
            }

            let current = grid[y][x];
            if neighbors.into_iter().all(|n| n > current) {
                risk += 1 + current
            }
        }
    }

    risk
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");

    let grid = parse_input(&contents);

    println!("Part 1: {}", part1(&grid));
}
