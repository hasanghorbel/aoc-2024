use std::{fs::File, io::Read};

fn take_input() -> String {
  let mut input = String::new();
  File::open("./src/example.txt")
      .unwrap()
      .read_to_string(&mut input)
      .unwrap();
  input
}

pub fn part2() {
  let input = take_input();
  
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