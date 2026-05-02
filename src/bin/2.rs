use std::cmp::min;

fn main() {
    let input = include_str!("../../data/2.txt").trim();
    let a = a(input);
    println!("a: {a}");
    let b = b(input);
    println!("b: {b}");
}

fn dimensions(box_line: &str) -> [i32; 3] {
    box_line
        .split('x')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn required_paper(box_line: &str) -> i32 {
    let [w, l, h] = dimensions(box_line);

    2 * (l * w + w * h + h * l) + min(l * w, min(w * h, h * l))
}

fn required_ribbon(box_line: &str) -> i32 {
    let mut dims = dimensions(box_line);
    dims.sort_unstable();
    let [a, b, c] = dims;
    2 * (a + b) + a * b * c
}

fn a(input: &str) -> i32 {
    input.split("\n").map(required_paper).sum()
}

fn b(input: &str) -> i32 {
    input.split("\n").map(required_ribbon).sum()
}
