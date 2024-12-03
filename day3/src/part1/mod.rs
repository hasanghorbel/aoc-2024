use regex::Regex;

pub fn part1() -> usize {
  let input = include_str!("../input.txt");
  let re = Regex::new(r"mul\(([0-9]{1,3})+,([0-9]{1,3})+\)").unwrap();
  re.captures_iter(input)
        .map(|c| c[1].parse::<usize>().unwrap() * c[2].parse::<usize>().unwrap())
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