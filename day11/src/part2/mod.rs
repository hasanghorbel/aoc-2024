use std::collections::HashMap;

pub fn part2() -> usize {
    let input = include_str!("../input.txt")
        .split(' ')
        .map(|stone| stone.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut stones = input.into_iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    for _ in 0..75 {
        stones = blink(&stones);
    }
    stones.values().sum()
}
fn blink(stones: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new = HashMap::new();
    stones.into_iter().for_each(|(&stone, &count)| {
        if stone == 0 {
            *new.entry(1).or_insert(0) += count;
        } else {
            let stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                let (left, right) = stone_str.split_at(stone_str.len() / 2);
                *new.entry(left.parse().unwrap()).or_insert(0) += count;
                *new.entry(right.parse().unwrap()).or_insert(0) += count;
            } else {
                *new.entry(stone * 2024).or_insert(0) += count;
            }
        }
    });
    new
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
