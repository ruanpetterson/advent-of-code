use std::collections::HashMap;

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

    println!(
        "What is the total distance between your lists? {total_distance}",
        total_distance = total_distance(&left, &right)
    );

    println!("--- Part Two ---");
    println!(
        "Once again consider your left and right lists. What is their \
         similarity score? {similarity}",
        similarity = similarity(&left, &right)
    );
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

fn total_distance(left: &[i32], right: &[i32]) -> i32 {
    left.iter()
        .zip(right.iter())
        .fold(0, |acc, (l, r)| (l - r).abs() + acc)
}

fn similarity(left: &[i32], right: &[i32]) -> i32 {
    #[derive(Clone, Copy)]
    struct Similarity {
        count: i32,
        value: i32,
    }

    let mut similarities: HashMap<i32, Similarity> = HashMap::new();
    for i in left {
        if let Some(sim) = similarities.get_mut(&i) {
            sim.count += 1;
            continue;
        }

        let value = right.iter().filter(|&j| i == j).copied().sum();

        similarities.insert(*i, Similarity { count: 1i32, value });
    }

    similarities
        .into_values()
        .fold(0, |acc, Similarity { count, value }| acc + count * value)
}
