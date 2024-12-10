use std::collections::{HashMap, HashSet};

pub fn part2() -> usize {
    let input: Vec<Vec<char>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antidotes = HashSet::new();
    let width = input[0].len();
    let height = input.len();
    for i in 0..height {
        for j in 0..width {
            if input[i][j] != '.' {
                map.entry(input[i][j])
                    .and_modify(|pos| {
                        if !pos.contains(&(j, i)) {
                            pos.push((j, i));
                        }
                    })
                    .or_insert(vec![(j, i)]);
            }
        }
    }

    for value in map.clone().values() {
        for i in 0..value.len() {
            for j in (i + 1)..value.len() {
                let (a, b) = value[i];
                let (c, d) = value[j];
                let slope = (d as f64 - b as f64) / (c as f64 - a as f64);
                let remainder = b as f64 - a as f64 * slope;
                let mut y1 = b.min(d) as isize;
                let mut y2 = b.max(d) as isize;
                let y_spacing = y2 - y1;

                while y1 >= 0 {
                    let x1 = ((y1 as f64 - remainder) / slope).round() as isize;
                    if 0 <= x1 && x1 < width as isize {
                        antidotes.insert((x1, y1));
                    }

                    y1 -= y_spacing;
                }
                while y2 < height as isize {
                    let x2 = ((y2 as f64 - remainder) / slope).round() as isize;
                    if 0 <= x2 && x2 < width as isize {
                        antidotes.insert((x2, y2));
                    }

                    y2 += y_spacing;
                }
            }
        }
    }
    antidotes.len()
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
