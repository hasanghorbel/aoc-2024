struct Robot {
    p: (isize, isize),
    v: (isize, isize),
}

impl From<&str> for Robot {
    fn from(line: &str) -> Self {
        let (pos, velocity) = line[2..].split_once(" v=").unwrap();
        let (x, y) = pos.split_once(',').unwrap();
        let (vx, vy) = velocity.split_once(',').unwrap();
        Self {
            p: (x.parse().unwrap(), y.parse().unwrap()),
            v: (vx.parse().unwrap(), vy.parse().unwrap()),
        }
    }
}

impl Robot {
    fn predict(&mut self, secs: isize) {
        self.p.0 = (self.p.0 + secs * self.v.0).rem_euclid(WIDTH);
        self.p.1 = (self.p.1 + secs * self.v.1).rem_euclid(HEIGHT);
    }
    fn is_safe(&self) -> bool {
        self.p.0 != WIDTH / 2 && self.p.1 != HEIGHT / 2
    }
    fn quadrant(&self) -> usize {
        if self.p.0 < WIDTH / 2 {
            if self.p.1 < HEIGHT / 2 {
                return 0;
            }
            return 1;
        } else if self.p.1 < HEIGHT / 2 {
            return 2;
        }
        3
    }
}

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

pub fn part1() -> isize {
    let input = include_str!("../input.txt");
    let mut quadrants = vec![0; 4];
    input.lines().for_each(|line| {
        let mut robot = Robot::from(line);
        robot.predict(100);

        if robot.is_safe() {
            quadrants[robot.quadrant()] += 1;
        }
    });
    quadrants.into_iter().product()
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
