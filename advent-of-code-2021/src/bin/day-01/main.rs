use std::collections::LinkedList;

const INPUT: &str = include_str!("./input.txt");

fn main() -> Result<(), std::io::Error> {
    let measures: Vec<_> =
        INPUT.lines().filter_map(|line| line.parse().ok()).collect();

    let mut sonar = LinkedList::new();
    let mut puzzle_a = 0;
    let mut puzzle_b = 0;

    measures.iter().for_each(|&measure| {
        if let Some(&prev) = sonar.back() {
            if measure > prev {
                puzzle_a += 1;
            }
        }

        if sonar.len() == 3 {
            let prev = sonar.iter().sum::<i32>();
            sonar.pop_front();
            let curr = sonar.iter().sum::<i32>() + measure;

            if curr > prev {
                puzzle_b += 1;
            }
        }

        sonar.push_back(measure);
    });

    dbg!(puzzle_a);
    dbg!(puzzle_b);

    Ok(())
}
