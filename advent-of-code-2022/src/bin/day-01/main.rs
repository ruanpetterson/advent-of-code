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
    /// Inserts an item into a sorted slice at the appropriate position based on
    /// a given predicate.
    ///
    /// The `insert_by` function takes a mutable reference to a slice, an item
    /// to insert, and a predicate that compares elements of the slice with
    /// the item. It finds the index in the slice where the item
    /// should be inserted based on the ordering defined by the predicate and
    /// shifts the elements after that index up by one position. The item is
    /// then inserted at the determined index.
    ///
    /// If the slice is not sorted or if the comparator function does not
    /// implement an order consistent with the sort order of the underlying
    /// slice, the returned result is unspecified and meaningless.
    ///
    /// # Examples
    ///
    /// ```
    /// # use core::cmp::Ord;
    /// let mut numbers = [1, 3, 5, 7, 9];
    ///
    /// insert_by(&mut numbers, 4, <i32 as Ord>::cmp);
    ///
    /// assert_eq!(numbers, [1, 3, 4, 5, 7]);
    /// ```
    #[inline(always)]
    fn insert_by<T, F>(slice: &mut [T], element: T, predicate: F)
    where
        F: Fn(&T, &T) -> core::cmp::Ordering,
    {
        // Find the index that satisfies the predicate.
        let Some(index) = slice.iter().position(|x| predicate(x, &element).is_ge()) else {
            return; // No-op if predicate condition is not satisfied.
        };

        unsafe {
            // SAFETY: This block of code is unsafe because it directly
            // manipulates raw pointers and performs
            // pointer arithmetic. However, it is considered safe under the
            // following conditions:
            // - The given slice is guaranteed to be valid, have proper
            //   alignment, and contain properly initialized elements as Rust
            //   guarantees these properties for slices.
            // - The index is within the bounds of the slice length or equal to
            //   the slice length. Therefore, the code does not access or modify
            //   memory outside the bounds of the slice.

            // Shift all elements after the index up one position
            let ptr = slice.as_mut_ptr().add(index);
            core::ptr::copy(ptr, ptr.add(1), slice.len() - index - 1);

            // Insert the new element at the index
            core::ptr::write(ptr, element);
        }
    }

    // Create an array to populate
    it.fold([Elf::default(); N], |mut list, elf| {
        insert_by(&mut list, elf, |a, b| a.cmp(b).reverse());
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
