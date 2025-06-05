use itertools::Itertools as _;

fn main() {
    static INPUT: &str = include_str!("./input.txt");

    let mut left = vec![];
    let mut right = vec![];

    for [l, r] in pairs(INPUT) {
        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    println!("--- Day 1: Calorie Counting ---");

    let total_distance = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (l, r)| (l - r).abs() + acc);
    println!("What is the total distance between your lists? {total_distance}");
}

fn pairs(input: &str) -> impl Iterator<Item = [i32; 2]> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .collect_array::<2>()
                .expect("BUG: expected pair")
        })
        .map(|pair| {
            pair.map(|n| n.parse::<i32>().expect("BUG: expected number"))
        })
}
