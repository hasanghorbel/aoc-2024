use std::{
    collections::VecDeque,
    ops::{Add, Index},
    usize,
};

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Empty,
    Wall,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    fn reachable(&self, (row, col): (usize, usize)) -> bool {
        match self {
            Direction::Down => row < LENGTH - 1,
            Direction::Up => row > 0,
            Direction::Left => col > 0,
            Direction::Right => col < LENGTH - 1,
        }
    }
}

impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);
    fn add(self, rhs: Direction) -> Self::Output {
        let (row, col) = self;
        match rhs {
            Direction::Down => (row + 1, col),
            Direction::Up => (row - 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        }
    }
}

struct Memory {
    grid: Vec<Vec<Tile>>,
}

impl Index<(usize, usize)> for Memory {
    type Output = Tile;
    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        &self.grid[row][col]
    }
}

impl From<&str> for Memory {
    fn from(value: &str) -> Self {
        let mut grid = vec![vec![Tile::Empty; LENGTH]; LENGTH];
        value.lines().take(BYTES).for_each(|line| {
            let (x, y): (usize, usize) = line
                .split_once(',')
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();
            grid[y][x] = Tile::Wall;
        });

        Self { grid }
    }
}

impl Memory {
    fn bfs(&self) -> usize {
        let start = (0, 0);
        let end = (LENGTH - 1, LENGTH - 1);

        let mut visited = [[false; LENGTH]; LENGTH];
        let mut queue = VecDeque::new();

        queue.push_back((start, 0));
        while let Some((pos, weight)) = queue.pop_front() {
            if pos == end {
                return weight;
            }
            let (row, col) = pos;

            if visited[row][col] {
                continue;
            }

            visited[row][col] = true;
            for dir in Direction::ALL {
                if dir.reachable(pos) {
                    let new_pos = pos + dir;

                    if self[new_pos] == Tile::Empty {
                        queue.push_back((new_pos, weight + 1));
                    }
                }
            }
        }
        panic!("no path is found");
    }
}

const LENGTH: usize = 71;
const BYTES: usize = 1024;

pub fn part1() -> usize {
    let input = include_str!("../input.txt");
    let memory = Memory::from(input);
    memory.bfs()
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
