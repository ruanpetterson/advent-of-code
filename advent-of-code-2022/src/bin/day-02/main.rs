struct HandError;

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn points(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn win(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn loss(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn draw(&self) -> Self {
        *self
    }

    fn outcome(&self, other: Self) -> Outcome {
        match (self, other) {
            (Self::Rock, Self::Paper) => Outcome::Loss,
            (Self::Rock, Self::Scissors) => Outcome::Win,
            (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Paper, Self::Scissors) => Outcome::Loss,
            (Self::Scissors, Self::Rock) => Outcome::Loss,
            (Self::Scissors, Self::Paper) => Outcome::Win,
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

impl Outcome {
    fn points(&self) -> usize {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    fn matching_hand(&self, their: Hand) -> Hand {
        match self {
            Outcome::Loss => their.win(),
            Outcome::Draw => their.draw(),
            Outcome::Win => their.loss(),
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

fn read_to_hands_part_1(
    input: &str,
) -> impl Iterator<Item = (Hand, Hand)> + '_ {
    input.lines().filter_map(|line| {
        let hands = line
            .split_whitespace()
            .filter_map(|round| round.chars().next())
            .collect::<Vec<_>>();

        match hands.as_slice() {
            &[their, our] => {
                Some((Hand::try_from(their).ok()?, Hand::try_from(our).ok()?))
            }
            _ => None,
        }
    })
}

fn read_to_hands_part_2(
    input: &str,
) -> impl Iterator<Item = (Hand, Hand)> + '_ {
    input.lines().filter_map(|line| {
        let hands = line
            .split_whitespace()
            .filter_map(|round| round.chars().next())
            .collect::<Vec<_>>();

        match hands.as_slice() {
            &[their, our] => {
                let their = Hand::try_from(their).ok()?;
                let our = Outcome::try_from(our).ok()?.matching_hand(their);

                Some((their, our))
            }
            _ => None,
        }
    })
}

fn main() {
    const INPUT: &str = include_str!("./input.txt");
    println!("--- Day 2: Rock Paper Scissors ---");
    let result = read_to_hands_part_1(INPUT).fold(0, |acc, (their, our)| {
        acc + our.outcome(their).points() + our.points()
    });
    println!("What would your total score be if everything goes exactly according to your strategy guide? {}", result);

    println!("--- Part Two ---");
    let result = read_to_hands_part_2(INPUT).fold(0, |acc, (their, our)| {
        acc + our.outcome(their).points() + our.points()
    });
    println!("What would your total score be if everything goes exactly according to your strategy guide? {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("./test.txt");

    #[test]
    fn part_1() {
        let result =
            read_to_hands_part_1(INPUT).fold(0, |acc, (their, our)| {
                acc + our.outcome(their).points() + our.points()
            });
        assert_eq!(15, result);
    }

    #[test]
    fn part_2() {
        let result =
            read_to_hands_part_2(INPUT).fold(0, |acc, (their, our)| {
                acc + our.outcome(their).points() + our.points()
            });
        assert_eq!(12, result);
    }
}
