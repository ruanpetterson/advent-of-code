#[derive(Debug)]
pub struct ParseError;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Forward(isize),
    Down(isize),
    Up(isize),
}

impl TryFrom<&str> for Direction {
    type Error = ParseError;

    fn try_from(movement: &str) -> Result<Self, Self::Error> {
        let [direction, length]: [&str; 2] = movement
            .split(' ')
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| ParseError)?;

        let length = length.parse::<isize>().map_err(|_| ParseError)?;

        match direction.to_ascii_lowercase().as_str() {
            "forward" => Ok(Self::Forward(length)),
            "down" => Ok(Self::Down(length)),
            "up" => Ok(Self::Up(length)),
            _ => Err(ParseError),
        }
    }
}

pub struct Submarine {
    position: [isize; 2],
    aim: Option<isize>,
}

impl Submarine {
    pub fn new(angular: bool) -> Self {
        Self {
            position: [0, 0],
            aim: if angular { Some(0) } else { None },
        }
    }

    pub fn position(&self) -> &[isize] {
        &self.position
    }
}

impl Default for Submarine {
    fn default() -> Self {
        Self {
            position: [0, 0],
            aim: None,
        }
    }
}

pub trait Dive {
    fn dive(&mut self, direction: Direction);
}

impl Dive for Submarine {
    fn dive(&mut self, direction: Direction) {
        if let Some(ref mut aim) = self.aim {
            match direction {
                Direction::Forward(distance) => {
                    let x = self.position.get_mut(0).unwrap();
                    *x += distance;

                    let y = self.position.get_mut(1).unwrap();
                    *y += distance * aim.to_owned();
                }
                Direction::Down(distance) => *aim += distance,
                Direction::Up(distance) => *aim -= distance,
            };
        } else {
            match direction {
                Direction::Forward(distance) => {
                    let x = self.position.get_mut(0).unwrap();
                    *x += distance
                }
                Direction::Down(distance) => {
                    let y = self.position.get_mut(1).unwrap();
                    *y += distance;
                }
                Direction::Up(distance) => {
                    let y = self.position.get_mut(1).unwrap();
                    *y -= distance;
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Direction, Dive, Submarine};

    #[test]
    fn it_works() {
        let movements = vec![
            Direction::Forward(5),
            Direction::Down(5),
            Direction::Forward(8),
            Direction::Up(3),
            Direction::Down(8),
            Direction::Forward(2),
        ];

        let mut submarine_a = Submarine::default();
        let mut submarine_b = Submarine::new(true);

        movements.iter().for_each(|&direction| {
            submarine_a.dive(direction);
            submarine_b.dive(direction);
        });

        assert_eq!(submarine_a.position().iter().product::<isize>(), 150);
        assert_eq!(submarine_b.position().iter().product::<isize>(), 900);
    }
}
