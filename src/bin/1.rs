fn main() {
    let input = include_str!("../../data/1.txt");
    let a = a(input);
    println!("a: {a}");
    let b = b(input);
    println!("b: {b}")
}

fn a(input: &str) -> i32 {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum()
}

fn b(input: &str) -> usize {
    let mut floor = 0;
    input
        .chars()
        .position(|c| {
            floor += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
            floor == -1
        })
        .unwrap()
        + 1
}
