use std::ops::Add;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Coordinate {
    x: i32,
    y: i32
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Direction{
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub struct Guard {
    pub direction: Direction,
    pub coordinate: Coordinate
}

impl Guard {
    pub fn take_step(&mut self) {
        let movement = self.get_movement();
        self.coordinate = self.coordinate + movement
    }

    pub fn rotate(&mut self) {
        self.direction = match self.direction {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        };
    }

    pub fn get_movement(&self) -> Coordinate {
        match self.direction {
            Direction::UP => Coordinate::new(0, -1),
            Direction::DOWN => Coordinate::new(0, 1),
            Direction::LEFT => Coordinate::new(-1, 0),
            Direction::RIGHT => Coordinate::new(1, 0),
        }
    }
}
pub struct Grid {
    pub contents: Vec<Vec<char>>,
    pub starting_position: Coordinate,
}

impl Grid {
    // pub fn get_starting_position(&self) -> Coordinate {
    //     for y in 0..self.contents.len() {
    //         for x in 0..self.contents[0].len() {
    //             if self.contents[y][x] == '^' {
    //                 return Coordinate::new(x as i32, y as i32)
    //             }
    //         }
    //     }
    //
    //     Coordinate::new(-1, -1)
    // }

    pub fn get_char_at(&self, coordinate: Coordinate) -> Option<&char> {
        if coordinate.y >= 0 && (coordinate.y as usize) < self.contents.len() &&
            coordinate.x >= 0 && (coordinate.x as usize) < self.contents[coordinate.y as usize].len() {
            Some(&self.contents[coordinate.y as usize][coordinate.x as usize])
        } else {
            None
        }
    }
}