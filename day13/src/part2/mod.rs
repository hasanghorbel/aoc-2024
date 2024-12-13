struct ClawMachine {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}

impl ClawMachine {
    fn new() -> Self {
        Self {
            a: (0, 0),
            b: (0, 0),
            prize: (0, 0),
        }
    }
    fn solve(&self) -> isize {
		let determinant = self.a.0  * self.b.1  - self.a.1  * self.b.0;
        if determinant == 0 {
            return 0;
        }
		let cx =  self.prize.0 + 10000000000000;
		let cy = self.prize.1 +10000000000000;

		let a_pressed = cx * self.b.1 - cy * self.b.0;
		let b_pressed = cy * self.a.0 - cx * self.a.1;
        
        if a_pressed % determinant != 0 || b_pressed % determinant != 0 {
            return 0;
        }
        3 * (a_pressed / determinant) + ( b_pressed / determinant)
    }
}


pub fn part2() -> isize {
let input = include_str!("../input.txt");

    let mut ans = 0;
    for group in input.split("\r\n\r\n") {
        let mut machine = ClawMachine::new();
        for line in group.lines() {
            let (left, right) = line.split_once(": ").unwrap();
            let coords = right.split_once(", ").unwrap();
            let x = coords.0[2..].parse().unwrap();
            let y = coords.1[2..].parse().unwrap();
            match left {
                "Button A" => machine.a = (x, y),
                "Button B" => machine.b = (x, y),
                "Prize" => machine.prize = (x, y),
                _ => unreachable!(),
            }
        }
        ans += machine.solve();
        
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
