use std::{collections::HashMap, mem::swap};

use itertools::Itertools;

pub fn part2() -> usize {
    let input = include_str!("../input.txt");

    let mut stones: HashMap<usize, usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .counts();

    let mut new: HashMap<usize, usize> = HashMap::new();

    for _ in 0..75 {
        for (stone, count) in stones.drain() {
            if stone == 0 {
                *new.entry(1).or_default() += count;
            } else {
                let digit_count = stone.ilog10() + 1;
                if digit_count % 2 == 0 {
                    let left = stone / 10usize.pow(digit_count / 2);
                    let right = stone % 10usize.pow(digit_count / 2);
                    *new.entry(left).or_default() += count;
                    *new.entry(right).or_default() += count;
                } else {
                    *new.entry(stone * 2024).or_default() += count;
                }
            }
        }

        swap(&mut stones, &mut new);
    }

    stones.values().sum()
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
