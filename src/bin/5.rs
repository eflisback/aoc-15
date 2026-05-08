use std::collections::HashSet;

fn main() {
    let input = include_str!("../../data/5.txt").trim();
    let a = a(input);
    println!("a: {a}");
}

fn has_three_vowels(line: &str) -> bool {
    let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
    line.chars().filter(|c| vowels.contains(c)).count() >= 3
}

fn has_same_letter_twice_in_a_row(line: &str) -> bool {
    line.chars().zip(line.chars().skip(1)).any(|(a, b)| a == b)
}

fn has_no_forbidden_substrings(line: &str) -> bool {
    let forbidden: HashSet<(char, char)> =
        HashSet::from([('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')]);
    line.chars()
        .zip(line.chars().skip(1))
        .all(|(a, b)| !forbidden.contains(&(a, b)))
}

fn a(input: &str) -> usize {
    input
        .split("\n")
        .filter(|line| {
            has_three_vowels(line)
                && has_same_letter_twice_in_a_row(line)
                && has_no_forbidden_substrings(line)
        })
        .count()
}
