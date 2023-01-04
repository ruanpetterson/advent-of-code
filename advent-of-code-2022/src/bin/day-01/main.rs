use std::collections::VecDeque;
use std::{cmp, iter};

type Calories = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Index(usize, Calories);

impl cmp::PartialOrd for Index {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}
impl cmp::Ord for Index {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

fn read_to_elves(input: &str) -> impl Iterator<Item = Calories> + '_ {
    let mut lines = input.lines();
    iter::from_fn(move || {
        lines
            .by_ref()
            .map_while(|line| line.parse::<usize>().ok())
            .reduce(|acc, curr| acc + curr)
    })
}

fn main() {
    static INPUT: &str = include_str!("./input.txt");

    let elves = read_to_elves(INPUT).enumerate().fold(
        VecDeque::with_capacity(4),
        |mut heap, elf| {
            // TODO: use array instead VecDeque and implement a way to
            // insert in the midle of array
            let value = Index(elf.0, elf.1);
            let (Ok(position) | Err(position)) = heap.binary_search(&value);
            heap.insert(position, value);
            if heap.len() == 4 {
                heap.pop_front();
            }
            heap
        },
    );

    println!("--- Day 1: Calorie Counting ---");
    if let Some(Index(nth, calories)) = elves.back() {
        println!("Find the Elf carrying the most Calories: {nth}");
        println!("How many total Calories is that Elf carrying? {calories}");
    }

    println!("--- Part Two ---");
    let top_three_sum = elves
        .iter()
        .take(3)
        .map(|Index(_, calories)| calories)
        .sum::<usize>();
    println!(
        "How many Calories are those Elves carrying in total? {top_three_sum}"
    );
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use test_case::test_case;

    use super::*;

    static INPUT: &str = include_str!("./test.txt");

    #[test_case(1, 4, 7000 + 8000 + 9000; "fourth Elf is top 1 carrying a total of 24000 Calories")]
    #[test_case(2, 3, 5000 + 6000; "third Elf is top 2 carrying a total of 11000 calories")]
    #[test_case(3, 5, 10000; "fifth Elf is top 3 carrying one food item with 10000 Calories")]
    #[test_case(4, 1, 1000 + 2000 + 3000; "first Elf is top 4 carrying food a total of 6000 Calories")]
    #[test_case(5, 2, 4000; "second Elf is top 5 carrying one food item with 4000 Calories")]
    fn top(top_n: usize, index: usize, expected_sum: usize) {
        let sorted_elves = {
            read_to_elves(INPUT)
                .enumerate()
                .map(|(k, v)| cmp::Reverse(Index(k, v)))
                .collect::<BTreeSet<_>>()
        };

        assert_eq!(
            sorted_elves
                .iter()
                .nth(top_n - 1)
                .map(|cmp::Reverse(v)| (v.0, v.1)),
            Some((index - 1, expected_sum))
        );
    }
}
