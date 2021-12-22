enum Snailfish {
    Pair(Box<Snailfish>, Box<Snailfish>),
    Value(u32),
}

fn parse_number(number: &str) {
    let mut stack: Vec<Snailfish> = vec![];

    for c in number.chars() {
        match c {
            ',' | '[' => (), // Our lists have known lengths, so we can ignore opening
            ']' => {
                let pair = Snailfish::Pair(
                    Box::new(stack.pop().unwrap()),
                    Box::new(stack.pop().unwrap()),
                );
                stack.push(pair);
            }
            d => stack.push(Snailfish::Value(d.to_digit(10).unwrap())),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
