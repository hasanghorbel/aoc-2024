pub fn part2() -> usize {
    let input = include_str!("../input.txt");
    let (v1, v2): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let (i, j) = line.split_once("   ").unwrap();
            let i = i.parse::<usize>().unwrap();
            let j = j.parse::<usize>().unwrap();

            (i, j)
        })
        .unzip();

    v1.into_iter()
        .map(|b| v2.iter().filter(|a| a == &&b).count() * b)
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
