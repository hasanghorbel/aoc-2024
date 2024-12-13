use itertools::Itertools;

struct ClawMachine {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}

impl ClawMachine {
    fn new(a: &str, b: &str, c: &str) -> Self {
        Self {
            a: get_x_y(a, '+'),
            b: get_x_y(b, '+'),
            prize: get_x_y_changed(c, '='),
        }
    }
    fn solve(&self) -> (isize, isize) {
		let determinant = self.a.0  * self.b.1  - self.a.1  * self.b.0;
        if determinant == 0 {
            return (0, 0);
        }

		let a_pressed = self.prize.0 * self.b.1 - self.prize.1 * self.b.0;
		let b_pressed = self.prize.1 * self.a.0 - self.prize.0 * self.a.1;
        

        if a_pressed % determinant == 0 && b_pressed % determinant == 0 {
            return (a_pressed /determinant, b_pressed /determinant);
        }
        (0, 0)
    }
}

fn get_x_y(line: &str, c: char) -> (isize, isize) {
    let (left, right) = line.split_once(',').unwrap();
    let x = left.split_once(c).unwrap().1.parse::<isize>().unwrap();
    let y = right.split_once(c).unwrap().1.parse::<isize>().unwrap();
    (x, y)
}

fn get_x_y_changed(line: &str, c: char) -> (isize, isize) {
    let (left, right) = line.split_once(',').unwrap();
    let x = left.split_once(c).unwrap().1.parse::<isize>().unwrap() + 10000000000000;
    let y = right.split_once(c).unwrap().1.parse::<isize>().unwrap() + 10000000000000;
    (x, y)
}

pub fn part2() -> isize {
    let input = format!("{}\n\n", include_str!("../input.txt"));

    let mut ans = 0;

    for (a, b, c, _) in input.lines().tuples() {
        let machine = ClawMachine::new(a, b, c);
        let (a_min, b_min) = ClawMachine::solve(&machine);
        ans += a_min * 3 + b_min;
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
