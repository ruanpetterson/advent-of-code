use std::{cmp, io::BufRead, iter};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Elf {
    index: usize,
    calories: usize,
}

impl cmp::PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.calories.partial_cmp(&other.calories)
    }
}
impl cmp::Ord for Elf {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.calories.cmp(&other.calories)
    }
}

/// Read input to a functional iterator.
fn read_to_elves_iter<'i>(
    input: impl BufRead + 'i,
) -> impl Iterator<Item = Elf> + 'i {
    let mut lines = input.lines();
    iter::from_fn(move || {
        lines
            .by_ref()
            // Get only valid lines
            .filter_map(|line| line.ok())
            // Parse it to usize
            .map_while(|line| line.parse::<usize>().ok())
            // Sum it
            .reduce(|acc, curr| acc + curr)
    })
    .enumerate()
    // Map it to indexed elves
    .map(|(index, calories)| Elf { index, calories })
}

/// Create a const sized and ordered array from an Elf iterator.
/// This way we can get a top N easier.
fn iter_to_ordered_list<const N: usize>(
    it: impl Iterator<Item = Elf>,
) -> [Elf; N] {
    /// Inserts an element at position `index` within the slice, shifting all
    /// elements after it to the right, overwriting the last one.
    #[inline(always)]
    fn try_insert_at<T>(slice: &mut [T], index: usize, element: T) {
        if index < slice.len() {
            // SAFETY: this is safe, trust me!
            unsafe {
                let p = slice.as_mut_ptr().add(index);
                core::ptr::copy(p, p.add(1), slice.len() - index - 1);
                p.write(element);
            }
        } else {
            // Trying to insert out-of-bounds.
            // No-op.
        }
    }

    // Create an array to populate
    it.fold([Elf::default(); N], |mut list, elf| {
        // Find the correct index to put the new Elf.
        let (Ok(index) | Err(index)) =
            list.binary_search_by(|i| i.cmp(&elf).reverse());
        try_insert_at(list.as_mut_slice(), index, elf);
        list
    })
}

fn main() {
    static INPUT: &str = include_str!("./input.txt");

    let elves: [_; 3] =
        iter_to_ordered_list(read_to_elves_iter(INPUT.as_bytes()));

    println!("--- Day 1: Calorie Counting ---");
    if let Some(Elf { index, calories }) = elves.first() {
        println!("Find the Elf carrying the most Calories: {index}");
        println!("How many total Calories is that Elf carrying? {calories}");
    }

    println!("--- Part Two ---");
    let top_three_sum =
        elves.iter().take(3).map(|elf| elf.calories).sum::<usize>();
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
        let elves: [_; 5] =
            iter_to_ordered_list(read_to_elves_iter(INPUT.as_bytes()));

        assert_eq!(
            elves.iter().nth(top_n - 1),
            Some(&Elf {
                index: index - 1,
                calories: expected_sum
            })
        );
    }
}
