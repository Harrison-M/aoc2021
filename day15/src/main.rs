use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

struct Grid {
    base: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

fn parse_input(grid: &str) -> Grid {
    let base: Vec<Vec<u32>> = grid
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Found non-digit in input"))
                .collect()
        })
        .collect();

    let height = base.len();
    let width = base[0].len();

    Grid {
        base,
        height,
        width,
    }
}

fn neighbors((x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = vec![(x + 1, y), (x, y + 1)];
    if x > 0 {
        result.push((x - 1, y));
    }
    if y > 0 {
        result.push((x, y - 1));
    }
    result
}

impl Grid {
    fn get(&self, (x, y): (usize, usize), repeats: usize) -> Option<u32> {
        let x_repeat = x / self.width;
        let y_repeat = y / self.height;
        if x_repeat > repeats || y_repeat > repeats {
            None
        } else {
            Some(
                (self.base[y % self.height][x % self.width] + x_repeat as u32 + y_repeat as u32
                    - 1)
                    % 9
                    + 1,
            )
        }
    }

    fn distance_to_corner(&self, repeats: usize) -> u32 {
        // Dijkstra
        let mut distances: HashMap<(usize, usize), u32> = HashMap::new();
        distances.insert((0, 0), 0);

        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        let mut current: (usize, usize) = (0, 0);

        loop {
            let current_distance = *distances.get(&current).unwrap();
            if current
                == (
                    self.width * (repeats + 1) - 1,
                    self.height * (repeats + 1) - 1,
                )
            {
                break current_distance;
            }

            let unvisited_neighbors: Vec<_> = neighbors(current)
                .into_iter()
                .filter(|n| !visited.contains(n))
                .filter_map(|n| self.get(n, repeats).map(|d| (n, d)))
                .collect();

            for (neighbor, neighbor_distance) in unvisited_neighbors {
                let total_distance_to_neighbor = current_distance + neighbor_distance;
                distances
                    .entry(neighbor)
                    .and_modify(|d| *d = (*d).min(total_distance_to_neighbor))
                    .or_insert(total_distance_to_neighbor);
            }

            visited.insert(current);
            distances.remove(&current);

            current = *distances
                .iter()
                .min_by_key(|(_, d)| *d)
                .map(|(l, _)| l)
                .unwrap();
        }
    }
}

fn part1(grid: &Grid) -> u32 {
    grid.distance_to_corner(0)
}

fn part2(grid: &Grid) -> u32 {
    // Slow
    grid.distance_to_corner(4)
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
        assert_eq!(part1(&parse_input(SAMPLE)), 40);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(SAMPLE)), 315);
    }
}
