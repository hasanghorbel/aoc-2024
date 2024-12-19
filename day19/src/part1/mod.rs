use std::collections::{HashMap, HashSet};

fn is_possible<'a>(
    start: &'a str,
    towels: &HashSet<&str>,
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    if memo.contains_key(start) {
        return memo[start];
    }

    for i in 1..=start.len().min(8) {
        let prefix = &start[..i];
        let postfix = &start[i..];

        if towels.contains(prefix) {
            if is_possible(postfix, towels, memo) {
                memo.insert(start, true);
                return true;
            }
        }
    }
    memo.insert(start, false);
    false
}

pub fn part1() -> usize {
    let input: &str = include_str!("../input.txt");

    let (available, remaining) = input.split_once("\r\n\r\n").unwrap();

    let towels: HashSet<&str> = available.split(", ").collect();
    let mut cache = HashMap::new();
    cache.insert("", true);
    remaining
        .lines()
        .filter(|t| is_possible(t, &towels, &mut cache))
        .count()
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
