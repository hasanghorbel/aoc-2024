#[allow(unused)]
pub fn part1() -> usize {
  let input = include_str!("../example.txt");
  0
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