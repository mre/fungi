// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl From<u8> for Direction {
    fn from(n: u8) -> Self {
        use Direction::{East, North, South, West};
        return match n {
            0 => North,
            1 => East,
            2 => South,
            3 => West,
            _ => panic!("unacceptable direction"),
        };
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        return Self { x, y, d };
    }

    pub fn turn_right(self) -> Self {
        return Self {
            d: Direction::from((self.d as u8 + 1) % 4),
            ..self
        };
    }

    pub fn turn_left(self) -> Self {
        return Self {
            d: Direction::from((self.d as u8 + 3) % 4),
            ..self
        };
    }

    pub fn advance(self) -> Self {
        use Direction::{East, North, South, West};

        return match self.d {
            North => Self {
                y: self.y + 1,
                ..self
            },
            East => Self {
                x: self.x + 1,
                ..self
            },
            South => Self {
                y: self.y - 1,
                ..self
            },
            West => Self {
                x: self.x - 1,
                ..self
            },
        };
    }

    pub fn instructions(self, instructions: &str) -> Self {
        return instructions
            .chars()
            .fold(self, |robot, instruction| match instruction {
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                'R' => robot.turn_right(),
                _ => robot,
            });
    }

    pub fn position(&self) -> (i32, i32) {
        return (self.x, self.y);
    }

    pub fn direction(&self) -> &Direction {
        return &self.d;
    }
}
