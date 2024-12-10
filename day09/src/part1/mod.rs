pub fn part1() -> usize {
    let input: Vec<usize> = include_str!("../input.txt")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let mut blocks = vec![];
    let mut is_file = true;
    let mut id = 0;
    for num in input {
        if is_file {
            for _ in 0..num {
                blocks.push(Some(id));
            }
            id += 1;
        } else {
            for _ in 0..num {
                blocks.push(None);
            }
        }
        is_file = !is_file;
    }
    let (mut left, mut right) = (0, blocks.len() - 1);
    while left < right {
        while blocks[left].is_some() {
            left += 1;
        }
        while blocks[right].is_none() {
            right -= 1;
        }
        if left < right {
            blocks.swap(left, right);
        }
        left += 1;
        right -= 1;
    }
    blocks
        .into_iter()
        .enumerate()
        .filter_map(|(i, opt)| opt.map(|v| v * i))
        .sum()
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
