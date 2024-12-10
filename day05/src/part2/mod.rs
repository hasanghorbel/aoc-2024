use std::collections::HashMap;

struct PageRules {
    left: Vec<usize>,
    right: Vec<usize>,
}

pub fn part2() -> usize {
    let (rules, pages) = include_str!("../input.txt").split_once("\r\n\r\n").unwrap();

    let pages_vec: Vec<Vec<usize>> = pages
        .lines()
        .map(|l| {
            l.split(",")
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut page_rules = HashMap::<usize, PageRules>::new();

    rules.lines().into_iter().for_each(|line| {
        let mut parts = line.split('|');
        let left = parts.next().unwrap().trim().parse::<usize>().unwrap();
        let right = parts.next().unwrap().trim().parse::<usize>().unwrap();

        let x = page_rules.entry(left).or_insert(PageRules {
            left: vec![],
            right: vec![],
        });
        x.right.push(right);

        let y = page_rules.entry(right).or_insert(PageRules {
            left: vec![],
            right: vec![],
        });
        y.left.push(left);
    });

    let mut ans = 0;
    pages_vec.into_iter().for_each(|pages| {
        let mut test = false;
        for i in 0..pages.len() {
            let rules = page_rules.get(&pages[i]).unwrap();

            let left_vec = &pages[0..i];
            let right_vec = &pages[i + 1..pages.len()];

            if rules.left.iter().any(|&x| right_vec.contains(&x)) {
                test = true;
                break;
            };
            if rules.right.iter().any(|&x| left_vec.contains(&x)) {
                test = true;
                break;
            };
        }
        if test {
            // change in part2
            let mut pages = pages;
            pages.sort_by(|a, b| {
                if page_rules.get(a).unwrap().left.contains(b) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            ans += pages[pages.len() / 2];
        }
    });

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
