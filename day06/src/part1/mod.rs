use std::collections::HashSet;

pub fn part1() -> usize {
    let grid: Vec<Vec<char>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let width = grid[0].len();
    let height = grid.len();
    let mut guard_x = 0;
    let mut guard_y = 0;
    let mut hashtags = vec![];
    let mut visited = HashSet::new();
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == '^' {
                guard_x = j;
                guard_y = i;
            }
            if grid[i][j] == '#' {
                hashtags.push((j, i));
            }
        }
    }

    let mut pos = 0b100;
    loop {
        if pos == 0b100 {
            match hashtags
                .iter()
                .filter(|(x, y)| x == &guard_x && &guard_y > y)
                .min_by(|(_, y), (_, j)| j.cmp(y))
            {
                Some((_, y_new)) => {
                    let y = y_new + 1;

                    for j in y..=guard_y {
                        visited.insert((guard_x, j));
                    }
                    guard_y = y;
                }
                None => {
                    for j in 0..=guard_y {
                        visited.insert((guard_x, j));
                    }
                    break;
                }
            }

            pos = pos >> 1;
        } else if pos == 0b010 {
            match hashtags
                .iter()
                .filter(|(x, y)| y == &guard_y && &guard_x < x)
                .min_by(|(x, _), (i, _)| x.cmp(i))
            {
                Some((x_new, _)) => {
                    let x = x_new - 1;

                    for i in guard_x..=x {
                        visited.insert((i, guard_y));
                    }
                    guard_x = x;
                }
                None => {
                    for i in guard_x..=width - 1 {
                        visited.insert((i, guard_y));
                    }
                    break;
                }
            }

            pos = pos >> 1;
        } else if pos == 0b001 {
            match hashtags
                .iter()
                .filter(|(x, y)| x == &guard_x && &guard_y < y)
                .min_by(|(_, y), (_, j)| y.cmp(j))
            {
                Some((_, y_new)) => {
                    let y = y_new - 1;

                    for j in guard_y..=y {
                        visited.insert((guard_x, j));
                    }
                    guard_y = y;
                }
                None => {
                    for j in guard_y..=height {
                        visited.insert((guard_x, j));
                    }
                    break;
                }
            }

            pos = pos >> 1;
        } else {
            match hashtags
                .iter()
                .filter(|(x, y)| y == &guard_y && &guard_x > x)
                .min_by(|(x, _), (i, _)| i.cmp(x))
            {
                Some((x_new, _)) => {
                    let x = x_new + 1;

                    for i in x..=guard_x {
                        visited.insert((i, guard_y));
                    }
                    guard_x = x;
                }
                None => {
                    for i in 0..=guard_x {
                        visited.insert((i, guard_y));
                    }
                    break;
                }
            }

            pos = 0b100;
        }
    }

    visited.len()
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
