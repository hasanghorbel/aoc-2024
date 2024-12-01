use std::{fs::File, io::Read};

fn take_input() -> String {
  let mut input = String::new();
  File::open("./src/input.txt")
      .unwrap()
      .read_to_string(&mut input)
      .unwrap();
  input
}

pub fn part1() -> usize {
  let input = take_input();
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