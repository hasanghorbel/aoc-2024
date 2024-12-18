use std::{
    collections::VecDeque,
    ops::{Add, Index, IndexMut},
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
    fn check(&self, (row, col): (usize, usize)) -> bool {
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
impl IndexMut<(usize, usize)> for Memory {
    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
        &mut self.grid[row][col]
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
    fn corrupt_byte(&mut self, corrupted_byte: (usize, usize)) {
        self[corrupted_byte] = Tile::Wall;
    }
    fn bfs_reachable(&self) -> bool {
        let start = (0, 0);
        let end = (LENGTH - 1, LENGTH - 1);

        let mut visited = [[false; LENGTH]; LENGTH];
        let mut queue = VecDeque::new();

        queue.push_back((start, 0));
        while let Some((pos, weight)) = queue.pop_front() {
            if pos == end {
                return true;
            }
            let (row, col) = pos;

            if visited[row][col] {
                continue;
            }

            visited[row][col] = true;
            for dir in Direction::ALL {
                if dir.check(pos) {
                    let new_pos = pos + dir;

                    if self[new_pos] == Tile::Empty {
                        queue.push_back((new_pos, weight + 1));
                    }
                }
            }
        }
        false
    }
}

const LENGTH: usize = 71;
const BYTES: usize = 1024;

pub fn part2() -> (usize, usize) {
    let input = include_str!("../input.txt");
    let mut memory = Memory::from(input);

    let corrupted_bytes: Vec<(usize, usize)> = input
        .lines()
        .skip(BYTES)
        .map(|line| {
            line.split_once(',')
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap()
        })
        .collect();
    let mut idx = 0;
    while memory.bfs_reachable() {
        memory.corrupt_byte(corrupted_bytes[idx]);
        idx += 1
    }
    corrupted_bytes[idx - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ans = part2();
        println!("{:?}", ans);
    }
}
