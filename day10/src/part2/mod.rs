pub fn part2() -> usize {
    let input: Vec<Vec<usize>> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap_or(1) as usize)
                .collect()
        })
        .collect();
    let height = input.len();
    let width = input[0].len();
    let mut sum = 0;
    for i in 0..height {
        for j in 0..width {
            if input[i][j] == 0 {
                sum += trailhead(&input, i, j, 0, height, width);
            }
        }
    }
    sum
}

fn trailhead(
    grid: &Vec<Vec<usize>>,
    y: usize,
    x: usize,
    start: usize,
    h: usize,
    w: usize,
) -> usize {
    if start == 9 {
        return 1;
    }
    let next = start + 1;
    let mut s = 0;

    if x < w - 1 && grid[y][x + 1] == next {
        s += trailhead(grid, y, x + 1, next, h, w);
    }
    if x > 0 && grid[y][x - 1] == next {
        s += trailhead(grid, y, x - 1, next, h, w);
    }
    if y < h - 1 && grid[y + 1][x] == next {
        s += trailhead(grid, y + 1, x, next, h, w);
    }
    if y > 0 && grid[y - 1][x] == next {
        s += trailhead(grid, y - 1, x, next, h, w);
    }

    s
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
