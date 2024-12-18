use itertools::Itertools;
use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display, ops::Add, usize};

struct Maze {
    grid: Vec<Vec<Object>>,
    start: (usize, usize),
    end: (usize, usize),
    height: usize,
    width: usize,
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let mut grid: Vec<Vec<Object>> = Vec::new();
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

impl Maze {
    fn dijkstra(&self) -> usize {
        let mut min_cost = usize::MAX;
        let mut dist = vec![vec![usize::MAX; self.width]; self.height];
        let mut prio = BinaryHeap::new();
        dist[self.start.0][self.start.1] = 0;
        prio.push(Reverse(Tile {
            position: self.start,
            direction: Direction::East,
            cost: 0,
        }));
        while let Some(Reverse(Tile {
            position,
            direction,
            cost,
        })) = prio.pop()
        {
            let (row, col) = position;
            if position == self.end && cost < min_cost {
                min_cost = cost;
                continue;
            }
            if cost > dist[row][col] || cost >= min_cost {
                continue;
            }

            for dir in Direction::ALL {
                let next_dir = dir;
                let (next_row, next_col) = position + next_dir;
                let mut next_cost = cost;
                if next_dir == direction {
                    next_cost += 1;
                } else {
                    next_dir.rotate();
                    next_cost += 1001;
                }
                if self.grid[next_row][next_col] == Object::Wall {
                    continue;
                }

                if next_cost < dist[next_row][next_col] {
                    dist[next_row][next_col] = next_cost;
                    prio.push(Reverse(Tile {
                        position: (next_row, next_col),
                        direction: next_dir,
                        cost: next_cost,
                    }));
                }
            }
        }
        min_cost
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    North,
    East,
    West,
    South,
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    fn rotate(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}
impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, rhs: Direction) -> Self::Output {
        let (row, col) = self;
        match rhs {
            Direction::North => (row - 1, col),
            Direction::East => (row, col + 1),
            Direction::South => (row + 1, col),
            Direction::West => (row, col - 1),
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
    direction: Direction,
    cost: usize,
}

pub fn part1() -> usize {
    let input = include_str!("../input.txt");
    let maze = Maze::from(input);
    maze.dijkstra()
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
