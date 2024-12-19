use std::collections::{HashMap, HashSet};

fn get_combs<'a>(
    start: &'a str,
    towels: &HashSet<&str>,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if memo.contains_key(start) {
        return memo[start];
    }
    let mut combs = 0;
    for i in 1..=start.len().min(8) {
        let prefix = &start[..i];
        let postfix = &start[i..];

        if towels.contains(prefix) {
            combs += get_combs(postfix, towels, memo);
        }
    }
    memo.insert(start, combs);
    combs
}

pub fn part2() -> usize {
    let input: &str = include_str!("../input.txt");

    let (available, remaining) = input.split_once("\r\n\r\n").unwrap();

    let towels: HashSet<&str> = available.split(", ").collect();
    let mut cache = HashMap::new();
    cache.insert("", 1);
    remaining
        .lines()
        .map(|t| get_combs(t, &towels, &mut cache))
        .sum()
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
