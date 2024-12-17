use std::fmt::Display;
use std::mem;
use std::ops::Add;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Object {
    Empty,
    Wall,
    Box,
    Robot,
    BoxLeft,
    BoxRight,
}

impl From<char> for Object {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            'O' => Self::Box,
            '@' => Self::Robot,
            '[' => Self::BoxLeft,
            ']' => Self::BoxRight,
            _ => Self::Empty,
        }
    }
}

impl Object {
    fn double(self) -> impl Iterator<Item = Self> {
        match self {
            Object::Empty => [Object::Empty, Object::Empty].into_iter(),
            Object::Wall => [Object::Wall, Object::Wall].into_iter(),
            Object::Box => [Object::BoxLeft, Object::BoxRight].into_iter(),
            Object::Robot => [Object::Robot, Object::Empty].into_iter(),
            Object::BoxLeft | Object::BoxRight => panic!(),
        }
    }
}

impl From<Object> for char {
    fn from(val: Object) -> Self {
        match val {
            Object::Empty => '.',
            Object::Wall => '#',
            Object::Box => 'O',
            Object::Robot => '@',
            Object::BoxLeft => '[',
            Object::BoxRight => ']',
        }
    }
}

struct Warehouse {
    grid: Vec<Vec<Object>>,
    robot: (usize, usize),
    width: usize,
    height: usize,
}

impl Warehouse {
    fn new(input: &str) -> Self {
        let mut grid: Vec<Vec<Object>> = Vec::new();
        let mut robot = (0, 0);

        for (row, line) in input.lines().enumerate() {
            let mut grid_row = Vec::new();

            for (col, char) in line.char_indices() {
                let object = Object::from(char);

                if object == Object::Robot {
                    robot = (row, col);
                }

                grid_row.push(object);
            }

            grid.push(grid_row);
        }

        let height = grid.len();
        let width = grid[0].len();

        Self {
            grid,
            robot,
            width,
            height,
        }
    }

    fn extend(&mut self) {
        let mut temp: Vec<Vec<Object>> = Vec::new();
        mem::swap(&mut self.grid, &mut temp);

        self.grid = temp
            .into_iter()
            .map(|row| row.into_iter().flat_map(Object::double).collect())
            .collect();

        self.width *= 2;
        self.robot.1 *= 2;
    }

    fn move_robot(&mut self, direction: Direction) {
        let (row, col) = self.robot;

        if self.can_move_object(row, col, direction) {
            self.move_object(row, col, direction);
            self.robot = self.robot + direction;
        }
    }

    fn move_object(&mut self, row: usize, col: usize, direction: Direction) {
        let (next_row, next_col) = (row, col) + direction;
        let next_object = self.grid[next_row][next_col];

        match next_object {
            Object::Empty => {
                self.grid[next_row][next_col] = self.grid[row][col];
                self.grid[row][col] = Object::Empty;
            }
            Object::Box => {
                self.move_object(next_row, next_col, direction);
                self.grid[next_row][next_col] = self.grid[row][col];
                self.grid[row][col] = Object::Empty;
            }
            Object::BoxRight => {
                self.move_object(next_row, next_col - 1, direction);
                self.move_object(next_row, next_col, direction);
                self.grid[next_row][next_col] = self.grid[row][col];
                self.grid[row][col] = Object::Empty;
            }
            Object::BoxLeft => {
                self.move_object(next_row, next_col + 1, direction);
                self.move_object(next_row, next_col, direction);
                self.grid[next_row][next_col] = self.grid[row][col];
                self.grid[row][col] = Object::Empty;
            }
            _ => panic!("Can't move object!"),
        }
    }

    fn can_move_object(&self, row: usize, col: usize, direction: Direction) -> bool {
        let (next_row, next_col) = (row, col) + direction;
        let next_object = self.grid[next_row][next_col];

        match next_object {
            Object::Empty => true,
            Object::Wall => false,
            Object::Box => self.can_move_object(next_row, next_col, direction),
            Object::BoxLeft => {
                if direction == Direction::Left {
                    self.can_move_object(next_row, next_col, direction)
                } else if direction == Direction::Right {
                    self.can_move_object(next_row, next_col + 1, direction)
                } else {
                    self.can_move_object(next_row, next_col + 1, direction)
                        && self.can_move_object(next_row, next_col, direction)
                }
            }
            Object::BoxRight => {
                if direction == Direction::Right {
                    self.can_move_object(next_row, next_col, direction)
                } else if direction == Direction::Left {
                    self.can_move_object(next_row, next_col - 1, direction)
                } else {
                    self.can_move_object(next_row, next_col - 1, direction)
                        && self.can_move_object(next_row, next_col, direction)
                }
            }
            Object::Robot => panic!(),
        }
    }

    fn gps_coordinate(row: usize, col: usize) -> usize {
        row * 100 + col
    }

    fn sum_gps_coordinates(&self) -> usize {
        let mut sum = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                if self.grid[row][col] == Object::Box || self.grid[row][col] == Object::BoxLeft {
                    sum += Self::gps_coordinate(row, col);
                }
            }
        }

        sum
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, rhs: Direction) -> Self::Output {
        let (row, col) = self;
        match rhs {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            '>' => Self::Right,
            '<' => Self::Left,
            'v' => Self::Down,
            _ => panic!("Direction not found!"),
        }
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self
            .grid
            .iter()
            .map(|row| row.iter().map(|&object| char::from(object)).collect::<String>())
            .join("\n");

        writeln!(f, "{string}")
    }
}



pub fn part2() -> usize {
    let input: &str = include_str!("../input.txt");

    let (warehouse, moves) = input.split_once("\r\n\r\n").unwrap();

    let mut warehouse = Warehouse::new(warehouse);
    let moves: Vec<Direction> = moves
        .lines()
        .flat_map(|line| line.chars())
        .map(Direction::from)
        .collect();

    warehouse.extend();

    for direction in moves {
        warehouse.move_robot(direction);
    }

    warehouse.sum_gps_coordinates()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ans = part2();
        println!("{}", ans);
    }
}
