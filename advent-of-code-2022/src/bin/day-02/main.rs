use std::io::BufRead;
use std::ops::Not;

struct HandError;

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    /// Returns how much points this `Hand` provides.
    pub const fn points(self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    /// Returns which `Hand` this can win.
    pub const fn win(self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    /// Returns which `Hand` this can lose.
    pub const fn loss(self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    /// Returns which `Hand` this can draw.
    pub const fn draw(self) -> Self {
        self
    }

    /// Returns the [`Outcome`] according to another `Hand`.
    pub const fn outcome(self, other: Self) -> Outcome {
        match (self, other) {
            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock) => Outcome::Loss,
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Not for Outcome {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Loss => Self::Win,
            Self::Draw => Self::Draw,
            Self::Win => Self::Loss,
        }
    }
}

impl Outcome {
    /// Returns how much points this `Outcome` provides.
    pub const fn points(self) -> usize {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    /// According to a [`Hand`], provide a second one that matches with
    /// current `Outcome`.
    pub const fn matching_hand(self, hand: Hand) -> Hand {
        match self {
            Outcome::Loss => hand.loss(),
            Outcome::Draw => hand.draw(),
            Outcome::Win => hand.win(),
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = HandError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Loss),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(HandError),
        }
    }
}

impl TryFrom<char> for Hand {
    type Error = HandError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(HandError),
        }
    }
}

impl From<Hand> for Outcome {
    fn from(hand: Hand) -> Self {
        match hand {
            Hand::Rock => Outcome::Loss,
            Hand::Paper => Outcome::Draw,
            Hand::Scissors => Outcome::Win,
        }
    }
}

/// Read input to a functional iterator.
fn read_to_hands<'i>(
    input: impl BufRead + 'i,
) -> impl Iterator<Item = (Hand, Hand)> + 'i {
    input.lines().filter_map(|line| {
        let [their, our]: [_; 2] = line
            .ok()?
            .split_whitespace()
            // Get only one char by split
            .filter_map(|round| round.chars().next())
            .collect::<Vec<_>>()
            // Transform it to [char, char]
            .try_into()
            .ok()?;

        // Try to cast to Hand
        let their = Hand::try_from(their).ok()?;
        let our = Hand::try_from(our).ok()?;

        Some((their, our))
    })
}

fn main() {
    const INPUT: &str = include_str!("./input.txt");

    println!("--- Day 2: Rock Paper Scissors ---");
    let result = read_to_hands(INPUT.as_bytes())
        .by_ref()
        .fold(0, |acc, (their, our)| {
            acc + our.outcome(their).points() + our.points()
        });
    println!(
        "What would your total score be if everything goes exactly according \
         to your strategy guide? {result}"
    );

    println!("--- Part Two ---");
    let result = read_to_hands(INPUT.as_bytes()).by_ref().fold(
        0,
        |acc, (their, our)| {
            // Convert Hand to Outcome and find matching_hand for it.
            let our = Outcome::from(our).not().matching_hand(their);
            acc + our.outcome(their).points() + our.points()
        },
    );
    println!(
        "What would your total score be if everything goes exactly according \
         to your strategy guide? {result}"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("./test.txt");

    #[test]
    fn part_1() {
        let result =
            read_to_hands(INPUT.as_bytes()).fold(0, |acc, (their, our)| {
                acc + our.outcome(their).points() + our.points()
            });
        assert_eq!(15, result);
    }

    #[test]
    fn part_2() {
        let result =
            read_to_hands(INPUT.as_bytes()).fold(0, |acc, (their, our)| {
                let our = Outcome::from(our).not().matching_hand(their);
                acc + our.outcome(their).points() + our.points()
            });
        assert_eq!(12, result);
    }
}
