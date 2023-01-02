use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Direction {
    type Err = ();

    // FIXME: remove .unwrap() and add a friendly Err
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split(" ").collect();
        let (dir, len) = (v[0], v[1]);

        match dir {
            "Forward" | "forward" => {
                Ok(Self::Forward(len.parse::<i32>().unwrap()))
            }
            "Down" | "down" => Ok(Self::Down(len.parse::<i32>().unwrap())),
            "Up" | "up" => Ok(Self::Up(len.parse::<i32>().unwrap())),
            _ => panic!(""),
        }
    }
}

pub struct Submarine {
    position: [i32; 2],
    aim: Option<i32>,
}

impl Submarine {
    pub fn new(angular: bool) -> Self {
        Self {
            position: [0, 0],
            aim: if angular { Some(0) } else { None },
        }
    }

    pub fn position(&self) -> &[i32] {
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

        assert_eq!(submarine_a.position().iter().product::<i32>(), 150);
        assert_eq!(submarine_b.position().iter().product::<i32>(), 900);
    }
}
