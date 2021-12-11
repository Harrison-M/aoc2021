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

fn walk_basin(x: usize, y: usize, grid: &mut [Vec<u32>]) -> usize {
    if grid[y][x] == 9 {
        return 0;
    }
    grid[y][x] = 9;
    let mut count = 1;
    if x != 0 {
        count += walk_basin(x - 1, y, grid);
    }
    if y != 0 {
        count += walk_basin(x, y - 1, grid);
    }
    if grid[y].get(x + 1).is_some() {
        count += walk_basin(x + 1, y, grid);
    }
    if grid.get(y + 1).is_some() {
        count += walk_basin(x, y + 1, grid);
    }
    count
}

fn part2(input: &[Vec<u32>]) -> usize {
    let mut grid: Vec<Vec<u32>> = input.into();
    let mut basin_sizes = Vec::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != 9 {
                basin_sizes.push(walk_basin(x, y, &mut grid))
            }
        }
    }
    basin_sizes.sort_unstable();
    basin_sizes
        .into_iter()
        .rev()
        .take(3)
        .reduce(|acc, basin| acc * basin)
        .unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");

    let grid = parse_input(&contents);

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = include_str!("sample");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(SAMPLE)), 15);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(SAMPLE)), 1134);
    }
}
