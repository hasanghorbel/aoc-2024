fn safe(v: &Vec<usize>) -> bool {
    let mut inc = true;
    let mut dec = true;
    let mut sorted_v = v.clone();
    for i in 1..v.len() {
        if v[i] <= v[i - 1] {
            inc = false
        }
        if v[i] >= v[i - 1] {
            dec = false
        }
    }
    if !inc && !dec {
        return false;
    }
    if dec {
        sorted_v.reverse();
    }

    for i in 1..sorted_v.len() {
        if sorted_v[i] - sorted_v[i - 1] > 3 {
            return false;
        }
    }
    true
}

pub fn part1() -> usize {
    let input = include_str!("../input.txt");
    let mut ans = 0;
    for line in input.lines().into_iter() {
        let mut parts = line
            .split(" ")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        if safe(&mut parts) {
            ans += 1;
        }
    }
    ans
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
