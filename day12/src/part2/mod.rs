use std::collections::VecDeque;

pub fn part2() -> usize {
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
            if corner_up_left(&grid, row, col, x) && !visited[row][col] {
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
    let mut sides = 0;
    let mut area = 0;

    let mut queue = VecDeque::new();
    queue.push_back((row, col));

    while let Some((new_r, new_c)) = queue.pop_front() {
        area += 1;

        sides += corners(grid, new_r, new_c, x, width, height) as usize;

        if new_r != 0 && grid[new_r - 1][new_c] == x {
            if !visited[new_r - 1][new_c] {
                queue.push_back((new_r - 1, new_c));
                visited[new_r - 1][new_c] = true;
            }
        }
        if new_c != 0 && grid[new_r][new_c - 1] == x {
            if !visited[new_r][new_c - 1] {
                queue.push_back((new_r, new_c - 1));
                visited[new_r][new_c - 1] = true;
            }
        }
        if new_r != height - 1 && grid[new_r + 1][new_c] == x {
            if !visited[new_r + 1][new_c] {
                queue.push_back((new_r + 1, new_c));
                visited[new_r + 1][new_c] = true;
            }
        }
        if new_c != width - 1 && grid[new_r][new_c + 1] == x {
            if !visited[new_r][new_c + 1] {
                queue.push_back((new_r, new_c + 1));
                visited[new_r][new_c + 1] = true;
            }
        }
    }

    sides * area
}
fn corner_up_left(grid: &[Vec<char>], row: usize, col: usize, x: char) -> bool {
    (row == 0 || grid[row - 1][col] != x) && (col == 0 || grid[row][col - 1] != x)
}
fn corners(grid: &[Vec<char>], row: usize, col: usize, x: char, width: usize, height: usize) -> u8 {
    let check_up = row == 0 || grid[row - 1][col] != x;
    let check_left = col == 0 || grid[row][col - 1] != x;
    let check_right = col == width - 1 || grid[row][col + 1] != x;
    let check_down = row == height - 1 || grid[row + 1][col] != x;
    let check_diag_up_right = row == 0 || col == width - 1 || grid[row - 1][col + 1] != x;
    let check_diag_up_left = row == 0 || col == 0 || grid[row - 1][col - 1] != x;
    let check_diag_down_right =
        row == height - 1 || col == width - 1 || grid[row + 1][col + 1] != x;
    let check_diag_down_left = row == height - 1|| col == 0 || grid[row + 1][col - 1] != x;
    (check_up && check_left) as u8
        + (check_up && check_right) as u8
        + (check_down && check_left) as u8
        + (check_down && check_right) as u8
		+ (!check_up && !check_left && check_diag_up_left) as u8
        + (!check_up && !check_right && check_diag_up_right) as u8
        + (!check_down && !check_left && check_diag_down_left) as u8
        + (!check_down && !check_right && check_diag_down_right) as u8

 
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ans = part2();
        println!("{}", ans);
    }
    #[test]
    fn test_corners() {
        let grid: Vec<Vec<char>> = include_str!("../example.txt")
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let height = grid.len();
        let width = grid[0].len();
        let mut new = grid.clone();
        for row in 0..height {
            for col in 0..width {
                if corners(&grid, row, col, grid[row][col], width, height) != 0 {
                    new[row][col] = corners(&grid, row, col, grid[row][col], width, height) as char;
                }
            }
        }
        for i in new {
            println!("{:?}", i);
        }
    }
}
