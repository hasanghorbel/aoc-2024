pub fn part1() -> usize {
  let input = include_str!("../input.txt");
  let (mut v1, mut v2) : (Vec<_>, Vec<_>) = input.lines().map(|line| {
    let (i, j) = line.split_once("   ").unwrap();
    let i = i.parse::<usize>().unwrap();
    let j = j.parse::<usize>().unwrap();

    (i, j)
  }).unzip();

  v1.sort();
  v2.sort();

  let mut ans = 0;
  for (i, j) in v1.into_iter().zip(v2.into_iter()) {
    ans += i.abs_diff(j);
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