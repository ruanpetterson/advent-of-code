use std::fs::File;
use std::io::{self, BufRead, BufReader};

use day_02::{Direction, Dive, Submarine};

fn main() -> Result<(), io::Error> {
    let file = File::open("day-02/input.txt")?;
    let buf = BufReader::new(file);

    let movements: Vec<Direction> = buf
        .lines()
        .map(|line| {
            line.expect("Could not parse line")
                .parse()
                .expect("Value is not a measure")
        })
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
