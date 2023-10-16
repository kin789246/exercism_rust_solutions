// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { position: (x,y), direction: d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let d = match self.direction {
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self { position: self.position, direction: d }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let d = match self.direction {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Self { position: self.position, direction: d }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let p = match self.direction {
            Direction::East => (self.position.0+1, self.position.1),
            Direction::North => (self.position.0, self.position.1+1),
            Direction::South => (self.position.0, self.position.1-1),
            Direction::West => (self.position.0-1, self.position.1),
        };
        Self { position: p, direction: self.direction }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, instr| {
            match instr {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => robot
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
