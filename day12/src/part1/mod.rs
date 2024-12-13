use std::collections::VecDeque;

pub fn part1() -> usize {
    let grid: Vec<Vec<char>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut visited = vec![vec![false; width]; height];

    let mut total_price = 0;
    for row in 0..height {
        for col in 0..width {
            let x = grid[row][col];
            if corner(&grid, row, col, x) && !visited[row][col] {
                total_price += blood_fill(&grid, row, col, &mut visited, x, height, width);
            }
        }
    }
    total_price
}
fn blood_fill(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    visited: &mut Vec<Vec<bool>>,
    x: char,
    height: usize,
    width: usize,
) -> usize {
    visited[row][col] = true;
    let mut perimeter = 0;
    let mut area = 0;

    let mut queue = VecDeque::new();
    queue.push_back((row, col));

    while let Some((new_r, new_c)) = queue.pop_front() {
        area += 1;
        if new_r != 0 && grid[new_r - 1][new_c] == x {
            if !visited[new_r - 1][new_c] {
                queue.push_back((new_r - 1, new_c));
                visited[new_r - 1][new_c] = true;
            }
        } else {
            perimeter += 1;
        }
        if new_c != 0 && grid[new_r][new_c - 1] == x {
            if !visited[new_r][new_c - 1] {
                queue.push_back((new_r, new_c - 1));
                visited[new_r][new_c - 1] = true;
            }
        } else {
            perimeter += 1;
        }
        if new_r != height - 1 && grid[new_r + 1][new_c] == x {
            if !visited[new_r + 1][new_c] {
                queue.push_back((new_r + 1, new_c));
                visited[new_r + 1][new_c] = true;
            }
        } else {
            perimeter += 1;
        }
        if new_c != width - 1 && grid[new_r][new_c + 1] == x {
            if !visited[new_r][new_c + 1] {
                queue.push_back((new_r, new_c + 1));
                visited[new_r][new_c + 1] = true;
            }
        } else {
            perimeter += 1;
        }
    }

    perimeter * area
}
fn corner(grid: &[Vec<char>], row: usize, col: usize, x: char) -> bool {
    (row == 0 || grid[row - 1][col] != x) && (col == 0 || grid[row][col - 1] != x)
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
