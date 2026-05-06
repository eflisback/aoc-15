use std::time::Instant;

fn main() {
    let input = include_str!("../../data/4.txt");

    let a_start = Instant::now();
    let a = a(input);
    println!("a: {a} ({:.2?})", a_start.elapsed());

    let b_start = Instant::now();
    let b = b(input);
    println!("b: {b} ({:.2?})", b_start.elapsed());
}

fn a(input: &str) -> i32 {
    let mut n = 0;
    loop {
        let hash = md5::compute(format!("{}{}", input, n));
        if hash[0] == 0 && hash[1] == 0 && (hash[2] & 0xF0) == 0 {
            return n;
        }
        n += 1;
    }
}

fn b(input: &str) -> i32 {
    let mut n = 0;
    loop {
        let hash = md5::compute(format!("{}{}", input, n));
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return n;
        }
        n += 1;
    }
}
