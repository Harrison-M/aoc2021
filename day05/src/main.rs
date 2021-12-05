use std::{collections::HashMap, env, fs};

type Line = ((isize, isize), (isize, isize));

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    point
                        .split(',')
                        .map(|n| n.parse().expect("Invalid number"))
                        .collect::<Vec<isize>>()
                })
                .collect::<Vec<Vec<isize>>>()
        })
        .map(|pts| ((pts[0][0], pts[0][1]), (pts[1][0], pts[1][1])))
        .collect()
}

fn part1(lines: &[Line]) -> usize {
    let horizontals: Vec<&Line> = lines.iter().filter(|((_, y1), (_, y2))| y1 == y2).collect();

    let verticals: Vec<&Line> = lines.iter().filter(|((x1, _), (x2, _))| x1 == x2).collect();

    let mut point_counts: HashMap<(isize, isize), usize> = HashMap::new();

    for ((x, y1), (_, y2)) in verticals {
        let range = if y1 < y2 { *y1..=*y2 } else { *y2..=*y1 };
        for y in range {
            point_counts
                .entry((*x, y))
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }

    for ((x1, y), (x2, _)) in horizontals {
        let range = if x1 < x2 { *x1..=*x2 } else { *x2..=*x1 };
        for x in range {
            point_counts
                .entry((x, *y))
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }

    point_counts.values().filter(|&&c| c >= 2).count()
}

fn part2(lines: &[Line]) -> usize {
    let verticals: Vec<&Line> = lines.iter().filter(|((x1, _), (x2, _))| x1 == x2).collect();
    let rest: Vec<&Line> = lines.iter().filter(|((x1, _), (x2, _))| x1 != x2).collect();

    let mut point_counts: HashMap<(isize, isize), usize> = HashMap::new();

    for ((x, y1), (_, y2)) in verticals {
        let range = if y1 < y2 { *y1..=*y2 } else { *y2..=*y1 };
        for y in range {
            point_counts
                .entry((*x, y))
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }

    for ((x1, y1), (x2, y2)) in rest {
        // According to problem constraint, slope will always be integral
        let slope = (y2 - y1) / (x2 - x1);
        let range = if x1 < x2 { *x1..=*x2 } else { *x2..=*x1 };
        let mut y = if x1 < x2 { *y1 } else { *y2 };
        for x in range {
            point_counts
                .entry((x, y))
                .and_modify(|c| *c += 1)
                .or_insert(1);

            y += slope;
        }
    }

    point_counts.values().filter(|&&c| c >= 2).count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");

    let lines = parse_input(&contents);

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("sample");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(SAMPLE)), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(SAMPLE)), 12);
    }
}
