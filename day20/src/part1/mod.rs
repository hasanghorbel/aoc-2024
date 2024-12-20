use itertools::Itertools;
use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display, ops::Add, usize};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
}
impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, rhs: Direction) -> Self::Output {
        let (row, col) = self;
        match rhs {
            Direction::Up => (row - 1, col),
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Object {
    Empty,
    Wall,
    Start,
    End,
}
impl From<char> for Object {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            _ => panic!("Please remove {} from your map", value),
        }
    }
}
impl From<&Object> for char {
    fn from(value: &Object) -> Self {
        match value {
            Object::Empty => '.',
            Object::Wall => '#',
            Object::Start => 'S',
            Object::End => 'E',
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Tile {
    position: (usize, usize),
    cost: usize,
}

struct Maze {
    grid: Vec<Vec<Object>>,
    start: (usize, usize),
    end: (usize, usize),
    height: usize,
    width: usize,
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let mut grid = Vec::new();
        let mut cheats = Vec::new();
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (row, line) in value.lines().enumerate() {
            let mut grid_row = Vec::new();

            for (col, char) in line.char_indices() {
                if char == 'S' {
                    start = (row, col);
                }
                if char == 'E' {
                    end = (row, col)
                }
                if char == '#' {
                    cheats.push((row, col));
                }

                grid_row.push(Object::from(char));
            }

            grid.push(grid_row);
        }

        let height = grid.len();
        let width = grid[0].len();

        Self {
            grid,
            start,
            end,
            height,
            width,
        }
    }
}
impl Maze {
    fn dijkstra(&self) -> Vec<Vec<usize>> {
        let mut min_cost = usize::MAX;
        let mut prio = BinaryHeap::new();
        let mut dists = vec![vec![usize::MAX; self.width]; self.height];
        dists[self.start.0][self.start.1] = 0;
        prio.push(Reverse(Tile {
            position: self.start,
            cost: 0,
        }));
        while let Some(Reverse(Tile { position, cost })) = prio.pop() {
            let (row, col) = position;
            if position == self.end && cost < min_cost {
                min_cost = cost;
                continue;
            }
            if cost > dists[row][col] || cost >= min_cost {
                continue;
            }

            for dir in Direction::ALL {
                let (next_row, next_col) = position + dir;
                let next_cost = cost + 1;

                if self.grid[next_row][next_col] == Object::Wall {
                    continue;
                }

                if next_cost < dists[next_row][next_col] {
                    dists[next_row][next_col] = next_cost;
                    prio.push(Reverse(Tile {
                        position: (next_row, next_col),
                        cost: next_cost,
                    }));
                }
            }
        }
        dists
    }
    fn get_cheats_count(&self, dists: &Vec<Vec<usize>>) -> usize {
        let mut counts = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                if self.grid[row][col] == Object::Wall {
                    continue;
                }
                if col < self.width - 2
                    && self.grid[row][col + 1] == Object::Wall
                    && self.grid[row][col + 2] == Object::Empty
                    && dists[row][col + 2] > dists[row][col]
                {
                    if dists[row][col + 2] - dists[row][col] >= 102 {
                        counts += 1;
                    }
                }
                if row < self.height - 2
                    && self.grid[row + 1][col] == Object::Wall
                    && self.grid[row + 2][col] == Object::Empty
                    && dists[row + 2][col] > dists[row][col]
                {
                    if dists[row + 2][col] - dists[row][col] >= 102 {
                        counts += 1;
                    }
                }
                if col > 1
                    && self.grid[row][col - 1] == Object::Wall
                    && self.grid[row][col - 2] == Object::Empty
                    && dists[row][col - 2] > dists[row][col]
                {
                    if dists[row][col - 2] - dists[row][col] >= 102 {
                        counts += 1;
                    }
                }
                if row > 1
                    && self.grid[row - 1][col] == Object::Wall
                    && self.grid[row - 2][col] == Object::Empty
                    && dists[row - 2][col] > dists[row][col]
                {
                    if dists[row - 2][col] - dists[row][col] >= 102 {
                        counts += 1;
                    }
                }
            }
        }
        counts
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|object| char::from(object))
                    .collect::<String>()
            })
            .join("\n");

        writeln!(f, "{string}")
    }
}

pub fn part1() -> usize {
    let input = include_str!("../input.txt");
    let maze = Maze::from(input);

    let dists = maze.dijkstra();
    
    maze.get_cheats_count(&dists)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ans = part1();
        println!("{}", ans);
    }
}
