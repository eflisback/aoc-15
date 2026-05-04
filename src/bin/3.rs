use std::{collections::HashSet, ops::Add};

fn main() {
    let input = include_str!("../../data/3.txt");
    let a = a(input);
    println!("a: {a}");
    let b = b(input);
    println!("b: {b}")
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn parse_direction(c: char) -> Point {
    match c {
        '>' => Point { x: 1, y: 0 },
        'v' => Point { x: 0, y: -1 },
        '<' => Point { x: -1, y: 0 },
        '^' => Point { x: 0, y: 1 },
        _ => Point { x: 0, y: 0 },
    }
}

fn a(input: &str) -> usize {
    let mut houses: HashSet<Point> = HashSet::new();
    let mut pos = Point { x: 0, y: 0 };
    houses.insert(pos);
    for c in input.chars() {
        pos = pos + parse_direction(c);
        houses.insert(pos);
    }
    houses.len()
}

fn b(input: &str) -> usize {
    let mut houses: HashSet<Point> = HashSet::new();
    let mut santa_pos = Point { x: 0, y: 0 };
    let mut robot_pos = Point { x: 0, y: 0 };
    houses.insert(santa_pos);
    for (i, c) in input.chars().enumerate() {
        let dir = parse_direction(c);
        if i % 2 == 0 {
            santa_pos = santa_pos + dir;
            houses.insert(santa_pos);
        } else {
            robot_pos = robot_pos + dir;
            houses.insert(robot_pos);
        }
    }
    houses.len()
}
