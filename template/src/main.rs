pub mod part1;
pub mod part2;

use part1::part1;
use part2::part2;

fn main() {
  use std::time::Instant;
  println!("Day ?");
  println!("---------------");
  let now = Instant::now();
  part1();
  let elapsed = now.elapsed();
  println!("part1 : {:.2?}", elapsed);
  let now = Instant::now();
  part2();
  let elapsed = now.elapsed();
  println!("part2 : {:.2?}", elapsed);
  println!("---------------");
}