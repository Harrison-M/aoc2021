use std::{
    collections::{HashMap, HashSet},
    env,
};

fn part1(min_y: i32, max_y: i32) -> i32 {
    let mut max = 0;
    let mut check = 1;
    loop {
        let mut next = -(check + 1);
        let mut pos = 0;
        loop {
            pos += next;
            if pos >= min_y && pos <= max_y {
                max = check;
                break;
            }
            if pos <= min_y {
                break;
            }
            next -= 1;
        }
        check += 1;
        if -check <= min_y {
            break;
        }
    }

    max * (max + 1) / 2
}

fn part2(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> usize {
    // Find y trajectories and valid steps for them
    let mut step_to_ys: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut max_step = 0;
    for y in min_y..-min_y {
        let mut pos = 0;
        let (mut step, mut next) = if y < 0 { (0, y) } else { (y * 2 + 1, -(y + 1)) };

        loop {
            pos += next;
            step += 1;
            if pos >= min_y && pos <= max_y {
                step_to_ys
                    .entry(step)
                    .and_modify(|v| v.push(y))
                    .or_insert_with(|| vec![y]);

                if step > max_step {
                    max_step = step;
                }
            }
            if pos <= min_y {
                break;
            }
            next -= 1;
        }
    }

    let mut trajectories: HashSet<(i32, i32)> = HashSet::new();
    // Find x trajectories and valid steps for them
    for x in 1..=max_x {
        if x * (x + 1) / 2 < min_x {
            // Falls short
            continue;
        }

        let mut pos = 0;
        let mut next = x;
        let mut step = 0;
        loop {
            step += 1;
            if step > max_step {
                break;
            }

            // Thanks to the euler check we know this is in range if we get here
            if next == 0 {
                for s in step..=max_step {
                    if let Some(ys) = step_to_ys.get(&s) {
                        for y in ys.iter() {
                            trajectories.insert((x, *y));
                        }
                    }
                }

                break;
            }

            pos += next;
            if pos > max_x {
                break;
            }

            if pos >= min_x && pos <= max_x {
                if let Some(ys) = step_to_ys.get(&step) {
                    for y in ys.iter() {
                        trajectories.insert((x, *y));
                    }
                }
            }

            next -= 1;
        }
    }

    trajectories.len()
}

fn main() {
    let mut args = env::args();
    args.next();
    let min_x: i32 = args.next().unwrap().parse().unwrap();
    let max_x: i32 = args.next().unwrap().parse().unwrap();
    let min_y: i32 = args.next().unwrap().parse().unwrap();
    let max_y: i32 = args.next().unwrap().parse().unwrap();

    println!("Part 1: {}", part1(min_y, max_y));
    println!("Part 2: {}", part2(min_x, max_x, min_y, max_y));
}
