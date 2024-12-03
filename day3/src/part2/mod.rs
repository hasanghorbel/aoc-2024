use regex::Regex;

pub fn part2() -> usize {
  let input = include_str!("../input.txt");
  let mut ans = 0;
  let mut enabled = true;

  let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap();
  let ge = Regex::new(r"\d{1,3}").unwrap();
  for pattern in re.find_iter(&input) {
    let pat = pattern.as_str();
    match pat {
      "do()" => enabled = true,
      "don't()" => enabled = false,
      _ => {
          if !enabled {
            continue;
          }
          let mut nums = ge.find_iter(pat);
          let a: usize = nums.next().unwrap().as_str().parse().unwrap();
          let b: usize = nums.next().unwrap().as_str().parse().unwrap();
          ans += a*b;
        }
    }
  }
  ans
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