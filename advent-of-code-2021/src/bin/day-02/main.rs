mod internals;
use internals::*;

const INPUT: &str = include_str!("./input.txt");

fn main() -> Result<(), std::io::Error> {
    let movements: Vec<Direction> = INPUT
        .lines()
        .map(|line| line.parse().expect("Value is not a measure"))
        .collect();

    let mut submarine_a = Submarine::default();
    let mut submarine_b = Submarine::new(true);

    movements.iter().for_each(|&direction| {
        submarine_a.dive(direction);
        submarine_b.dive(direction);
    });

    dbg!(submarine_a.position().iter().product::<i32>());
    dbg!(submarine_b.position().iter().product::<i32>());

    Ok(())
}
