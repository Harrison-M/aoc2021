use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

#[derive(Clone)]
enum Side {
    Left(Rc<RefCell<Snailfish>>),
    Right(Rc<RefCell<Snailfish>>),
}

impl Side {
    fn borrow(&self) -> Ref<Snailfish> {
        match &self {
            Left(n) | Right(n) => n.borrow(),
        }
    }

    fn clone_inside(&self) -> Rc<RefCell<Snailfish>> {
        match &self {
            Left(n) | Right(n) => n.clone(),
        }
    }

    fn explode(&self) -> Snailfish {
        match &self {
            Left(n) | Right(n) => n.replace(Snailfish::Value(0)),
        }
    }

    fn value(&self) -> u32 {
        match &self {
            Left(n) | Right(n) => match *n.borrow() {
                Snailfish::Value(v) => v,
                _ => panic!("Attempted to fetch value of non-value arm"),
            },
        }
    }
}

enum Snailfish {
    Pair(Side, Side),
    Value(u32),
}

use Side::*;

impl Snailfish {
    fn left(&self) -> &Side {
        match self {
            Self::Pair(l, _) => l,
            Self::Value(_) => panic!("Not a pair"),
        }
    }
    fn right(&self) -> &Side {
        match self {
            Self::Pair(_, r) => r,
            Self::Value(_) => panic!("Not a pair"),
        }
    }
}

fn parse_number(number: &str) -> Snailfish {
    let mut stack: Vec<Snailfish> = vec![];

    for c in number.chars() {
        match c {
            ',' | '[' => (), // Our lists have known lengths, so we can ignore opening
            ']' => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let pair = Snailfish::Pair(
                    Left(Rc::new(RefCell::new(left))),
                    Right(Rc::new(RefCell::new(right))),
                );
                stack.push(pair);
            }
            d => stack.push(Snailfish::Value(d.to_digit(10).unwrap())),
        }
    }

    stack.into_iter().next().unwrap()
}

fn check_explosions(
    side: Side,
    pairs: &mut Vec<Rc<RefCell<Snailfish>>>,
    sides: &mut Vec<Side>,
) -> bool {
    let mut explode = false;
    let exploded = match *side.borrow() {
        Snailfish::Value(_) => false,
        Snailfish::Pair(ref l, ref r) => {
            if pairs.len() == 4 {
                explode = true;
                true
            } else {
                pairs.push(side.clone_inside());
                sides.push(side.clone());
                let result = check_explosions(l.clone(), pairs, sides)
                    || check_explosions(r.clone(), pairs, sides);
                pairs.pop();
                sides.pop();
                result
            }
        }
    };

    if explode {
        let exploding_pair = side.explode();

        let mut left_check_pairs = pairs.clone();
        let mut left_check_sides = sides.clone();
        let mut current_side = side.clone();

        while let Some(pair) = left_check_pairs.pop() {
            if let Right(_) = current_side {
                let borrowed_pair = pair.borrow();
                let mut left_node = borrowed_pair.left().clone_inside();
                loop {
                    let next_node: Rc<RefCell<Snailfish>>;
                    match *left_node.borrow_mut() {
                        Snailfish::Pair(_, ref r) => next_node = r.clone_inside(),
                        Snailfish::Value(ref mut v) => {
                            *v += exploding_pair.left().value();
                            break;
                        }
                    }

                    left_node = next_node;
                }
            } else if let Some(side) = left_check_sides.pop() {
                current_side = side;
            } else {
                break;
            }
        }

        let mut right_check_pairs = pairs.clone();
        let mut right_check_sides = sides.clone();
        let mut current_side = side;

        while let Some(pair) = right_check_pairs.pop() {
            if let Right(_) = current_side {
                let borrowed_pair = pair.borrow();
                let mut right_node = borrowed_pair.right().clone_inside();
                loop {
                    let next_node: Rc<RefCell<Snailfish>>;
                    match *right_node.borrow_mut() {
                        Snailfish::Pair(ref l, _) => next_node = l.clone_inside(),
                        Snailfish::Value(ref mut v) => {
                            *v += exploding_pair.right().value();
                            break;
                        }
                    }

                    right_node = next_node;
                }
            } else if let Some(side) = right_check_sides.pop() {
                current_side = side;
            } else {
                break;
            }
        }
    }

    exploded
}

fn check_splits(number: Rc<RefCell<Snailfish>>) -> bool {
    let mut split_target: Option<u32> = None;
    let split = match *number.borrow() {
        Snailfish::Value(v) if v > 9 => {
            split_target = Some(v);
            true
        }
        Snailfish::Pair(ref l, ref r) => {
            check_splits(l.clone_inside()) || check_splits(r.clone_inside())
        }
        _ => false,
    };

    if let Some(v) = split_target {
        let half = v / 2;
        number.replace(Snailfish::Pair(
            Left(Rc::new(RefCell::new(Snailfish::Value(half)))),
            Right(Rc::new(RefCell::new(Snailfish::Value(v - half)))),
        ));
    }

    split
}

fn reduce_number(number: Rc<RefCell<Snailfish>>) {
    let mut pair_stack = vec![Rc::clone(&number)];
}

fn main() {
    println!("Hello, world!");
}
