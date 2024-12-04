pub fn part2() -> usize {
  let input: Vec<Vec<char>> = include_str!("../input.txt").lines().map(|line| line.chars().collect()).collect();
  let height = input.len();
  let width = input.first().unwrap().len();
  let mut ans = 0;
  for i in 1..height - 1 {
      for j in 1..width - 1 {
          if input[i][j] != 'A' {
              continue;
          }

          if (input[i - 1][j - 1] == 'M' && input[i + 1][j + 1] == 'S'
              || input[i - 1][j - 1] == 'S' && input[i + 1][j + 1] == 'M')
              && (input[i - 1][j + 1] == 'M' && input[i + 1][j - 1] == 'S'
                  || input[i - 1][j + 1] == 'S' && input[i + 1][j - 1] == 'M')
          {
              ans += 1;
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