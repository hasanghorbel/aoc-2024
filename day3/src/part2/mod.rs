use regex::Regex;

fn part1(x: &str) -> usize {
  let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
  re.captures_iter(x)
        .map(|c| c[1].parse::<usize>().unwrap() * c[2].parse::<usize>().unwrap())
        .sum()
}

pub fn part2() -> usize {
  let input = include_str!("../input.txt");

  input.split("do()")
  .map(|x| part1(x.split("don't()").next().unwrap()))
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