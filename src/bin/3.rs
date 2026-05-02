use std::{collections::HashSet, ops::Add};

fn main() {
    let input = include_str!("../../data/3.txt");
    let a = a(input);
    println!("a: {a}");
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

fn a(input: &str) -> usize {
    let mut houses: HashSet<Point> = HashSet::new();
    let mut pos = Point { x: 0, y: 0 };
    houses.insert(pos);
    input.chars().for_each(|c| {
        pos = pos
            + match c {
                '>' => Point { x: 1, y: 0 },
                'v' => Point { x: 0, y: -1 },
                '<' => Point { x: -1, y: 0 },
                '^' => Point { x: 0, y: 1 },
                _ => Point { x: 0, y: 0 },
            };
        houses.insert(pos);
    });
    houses.len()
}
