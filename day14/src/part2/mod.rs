use itertools::Itertools;
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

pub fn part2() -> usize {
    let input = include_str!("../input.txt");
    let mut viktor: Vec<Robot> = input.lines().map(|line| Robot::from(line)).collect();
    (1..=101 * 103)
        .map(|t: usize| {
            (
                t,
                viktor
                    .iter_mut()
                    .map(|r| {
                        r.predict(1);
                        r.quadrant()
                    })
                    .counts()
                    .values()
                    .product::<usize>(),
            )
        })
        .min_by(|&x, &y| x.1.cmp(&y.1))
        .unwrap()
        .0
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
