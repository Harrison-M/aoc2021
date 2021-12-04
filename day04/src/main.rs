use std::{collections::HashSet, env, fs};

#[derive(Clone)]
struct BingoGame {
    boards: Vec<Vec<Vec<usize>>>,
    numbers: Vec<usize>,
}

fn parse_input(input: &str) -> BingoGame {
    let mut sections = input.split("\n\n");
    let numbers_str = sections.next().expect("Empty input");
    let numbers: Vec<usize> = numbers_str
        .split(',')
        .map(|num| num.parse().expect("Invalid number drawn"))
        .collect();

    let boards: Vec<Vec<Vec<usize>>> = sections
        .map(|board| {
            board
                .lines()
                .map(|row| {
                    row.split(' ')
                        .filter(|n| !n.is_empty())
                        .map(|n| n.parse().expect("Invalid board number"))
                        .collect()
                })
                .collect()
        })
        .collect();

    BingoGame { numbers, boards }
}

fn board_is_winner(board: &Vec<Vec<usize>>, numbers: &HashSet<usize>) -> bool {
    board
        .iter()
        .any(|row| row.iter().all(|num| numbers.contains(num)))
        || (0..=4).any(|idx: usize| board.iter().all(|row| numbers.contains(&row[idx])))
}

fn part1(game: &BingoGame) -> usize {
    let mut ball_dispenser = game.numbers.iter();
    let mut drawn_numbers: HashSet<usize> = HashSet::new();

    loop {
        let ball = ball_dispenser.next().expect("Out of balls");
        drawn_numbers.insert(*ball);
        if drawn_numbers.len() < 5 {
            continue;
        }

        if let Some(board) = game
            .boards
            .iter()
            .find(|board| board_is_winner(board, &drawn_numbers))
        {
            break ball
                * board
                    .iter()
                    .flat_map(|row| row.iter())
                    .filter(|&num| !drawn_numbers.contains(num))
                    .copied()
                    .reduce(|acc, num| acc + num)
                    .unwrap();
        }
    }
}

fn part2(init_game: &BingoGame) -> usize {
    let mut ball_dispenser = init_game.numbers.iter();
    let mut drawn_numbers: HashSet<usize> = HashSet::new();
    let mut game = init_game.clone();

    loop {
        let ball = ball_dispenser.next().expect("Out of balls");
        drawn_numbers.insert(*ball);
        if drawn_numbers.len() < 5 {
            continue;
        }

        let pre_check_boards = game.boards.clone();

        game.boards = game
            .boards
            .into_iter()
            .filter(|board| !board_is_winner(board, &drawn_numbers))
            .collect();

        if game.boards.len() == 0 {
            break ball
                * pre_check_boards[0]
                    .iter()
                    .flat_map(|row| row.iter())
                    .filter(|&num| !drawn_numbers.contains(num))
                    .copied()
                    .reduce(|acc, num| acc + num)
                    .unwrap();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");

    let game = parse_input(&contents);

    println!("Part 1: {}", part1(&game));
    println!("Part 2: {}", part2(&game));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("sample");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(SAMPLE)), 4512);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(SAMPLE)), 1924);
    }
}
