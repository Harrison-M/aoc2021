use point_2d::Point2D;
use std::{collections::HashMap, env, fs};

#[derive(Clone)]
struct Octopus {
    flashed: bool,
    level: u32,
}

type Grid = HashMap<Point2D<isize>, Octopus>;

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, octopus)| {
                (
                    Point2D(x as isize, y as isize),
                    Octopus {
                        flashed: false,
                        level: octopus.to_digit(10).expect("Non-digit found"),
                    },
                )
            })
        })
        .collect()
}

fn step(grid: &mut Grid) {
    for (_, octopus) in grid.iter_mut() {
        octopus.level += 1;
    }

    loop {
        let update_points: Vec<_> = grid
            .iter_mut()
            .filter(|(_, octopus)| !octopus.flashed && octopus.level > 9)
            .flat_map(|(point, octopus)| {
                octopus.flashed = true;
                point.adjacent_points().into_iter()
            })
            .collect();

        if update_points.is_empty() {
            break;
        }

        for neighbor in update_points {
            grid.entry(neighbor).and_modify(|o| o.level += 1);
        }
    }
}

fn part1(input_grid: &Grid) -> usize {
    let mut flashes = 0;
    let mut grid = input_grid.clone();
    for _ in 0..100 {
        step(&mut grid);

        for (_, octopus) in grid.iter_mut().filter(|(_, octopus)| octopus.flashed) {
            octopus.flashed = false;
            octopus.level = 0;
            flashes += 1;
        }
    }
    flashes
}

fn part2(input_grid: &Grid) -> usize {
    let mut steps = 0;
    let mut grid = input_grid.clone();
    loop {
        steps += 1;
        step(&mut grid);

        if grid.iter().all(|(_, octopus)| octopus.flashed) {
            break;
        }

        for (_, octopus) in grid.iter_mut().filter(|(_, octopus)| octopus.flashed) {
            octopus.flashed = false;
            octopus.level = 0;
        }
    }
    steps
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
        assert_eq!(part1(&parse_input(SAMPLE)), 1656);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(SAMPLE)), 195);
    }
}
