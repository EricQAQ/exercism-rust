// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn next_direction(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn previous_direction(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

pub struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Position { x: x, y: y }
    }

    pub fn advance_to_direction(self, direction: &Direction) -> Self {
        match *direction {
            Direction::North => Position::new(self.x, self.y + 1),
            Direction::East => Position::new(self.x + 1, self.y),
            Direction::South => Position::new(self.x, self.y - 1),
            Direction::West => Position::new(self.x - 1, self.y),
        }
    }
}

pub struct Robot {
    position: Position,
    direction: Direction,
}

impl Robot {
    #[allow(unused_variables)]
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot { position: Position::new(x, y), direction: d}
    }

    pub fn turn_right(self) -> Self {
        Robot{ position: self.position, direction: self.direction.next_direction() }
    }

    pub fn turn_left(self) -> Self {
        Robot{ position: self.position, direction: self.direction.previous_direction() }
    }

    pub fn advance(self) -> Self {
        Robot{
            position: self.position.advance_to_direction(&self.direction),
            direction: self.direction
        }
    }

    #[allow(unused_variables)]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, ch| match ch {
            'L' => robot.turn_left(),
            'R' => robot.turn_right(),
            'A' => robot.advance(),
            _   => robot
        })
    }

    pub fn position(&self) -> (isize, isize) {
        (self.position.x, self.position.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
