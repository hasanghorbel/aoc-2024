fn is_reachable(target: usize, nums: &[usize]) -> bool {
    if nums.len() == 1 {
        return target == nums[0];
    }
    let (&last, rest) = nums.split_last().unwrap();
    if target % last == 0 && is_reachable(target / last, rest) {
        return true;
    }
    if target > last && is_reachable(target - last, rest) {
        return true;
    }
    let last_len = last.ilog10() + 1;
    let magnitude = 10usize.pow(last_len);
    let target_len = target.ilog10() + 1;
    let ending = target % magnitude;
    if target_len > last_len && last == ending && is_reachable(target / magnitude, rest) {
        return true;
    }
    false
}

pub fn part2() -> usize {
    let input = include_str!("../input.txt");
    let mut equations: Vec<(usize, Vec<usize>)> = Vec::new();
    input.lines().for_each(|line| {
        let (result, nums) = line.split_once(':').unwrap();
        let result = result.parse().unwrap();
        let nums = nums
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        equations.push((result, nums));
    });

    equations
        .into_iter()
        .filter(|(result, nums)| is_reachable(*result, nums))
        .map(|(result, _)| result)
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
