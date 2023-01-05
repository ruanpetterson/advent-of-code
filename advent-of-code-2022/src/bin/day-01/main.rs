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

fn read_to_elves_iter(input: &str) -> impl Iterator<Item = Calories> + '_ {
    let mut lines = input.lines();
    iter::from_fn(move || {
        lines
            .by_ref()
            .map_while(|line| line.parse::<usize>().ok())
            .reduce(|acc, curr| acc + curr)
    })
}

fn iter_to_list<const N: usize>(
    it: impl Iterator<Item = Calories>,
) -> [Index; N] {
    it.enumerate()
        .fold([Index(0, 0); N], |mut list, (nth, calories)| {
            let item = Index(nth, calories);
            let (Ok(index) | Err(index)) =
                list.binary_search_by(|i| i.cmp(&item).reverse());
            insert_at(list.as_mut_slice(), index, item);
            list
        })
}

fn insert_at<T>(slice: &mut [T], index: usize, element: T) {
    let len = slice.len();

    unsafe {
        let p = slice.as_mut_ptr().add(index);
        if index < len {
            core::ptr::copy(p, p.add(1), len - index);
        } else if index == len {
            // No copy needed
        } else {
            panic!("insertion index (is {index}) should be <= len (is {len})");
        }

        p.write(element);
    }
}

fn main() {
    static INPUT: &str = include_str!("./input.txt");

    let elves: [_; 3] = iter_to_list(read_to_elves_iter(INPUT));

    println!("--- Day 1: Calorie Counting ---");
    if let Some(Index(nth, calories)) = elves.first() {
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
    use test_case::test_case;

    use super::*;

    static INPUT: &str = include_str!("./test.txt");

    #[test_case(1, 4, 7000 + 8000 + 9000; "fourth Elf is top 1 carrying a total of 24000 Calories")]
    #[test_case(2, 3, 5000 + 6000; "third Elf is top 2 carrying a total of 11000 calories")]
    #[test_case(3, 5, 10000; "fifth Elf is top 3 carrying one food item with 10000 Calories")]
    #[test_case(4, 1, 1000 + 2000 + 3000; "first Elf is top 4 carrying food a total of 6000 Calories")]
    #[test_case(5, 2, 4000; "second Elf is top 5 carrying one food item with 4000 Calories")]
    fn top(top_n: usize, index: usize, expected_sum: usize) {
        let elves: [_; 5] = iter_to_list(read_to_elves_iter(INPUT));

        assert_eq!(
            elves.iter().nth(top_n - 1),
            Some(&Index(index - 1, expected_sum))
        );
    }
}
