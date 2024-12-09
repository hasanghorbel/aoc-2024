pub fn part2() -> usize {
    let input: Vec<usize> = include_str!("../input.txt")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let mut blocks = vec![];
    let mut is_file = true;
    let mut id = 0;
    for num in input {
        if is_file {
            for _ in 0..num {
                blocks.push(Some(id));
            }
            id += 1;
        } else {
            for _ in 0..num {
                blocks.push(None);
            }
        }
        is_file = !is_file;
    }

    id -= 1;
    let mut right = (blocks.len() - 1) as isize;
		while right > 0 {
			let mut n = right as usize;
			while blocks[n].is_none() || blocks[n] != Some(id) {
					n -= 1;
			}
			let mut last = 0;
			let mut left_cache = 0;
			let mut right_cache = 0;

			while n >= right_cache && blocks[n - right_cache].is_some() && blocks[n - right_cache] == Some(id) {
					right_cache += 1;
			}
			if id == 0 {
				break;
			}
			id -= 1;
			while left_cache < right_cache && last <= n {
					last += left_cache;
					left_cache = 0;
					while blocks[last].is_some() {
							last += 1;
					}
					while last + left_cache <= n && blocks[last + left_cache].is_none() {
							left_cache += 1;
					}
			}


			if left_cache >= right_cache {
					for l in 0..right_cache {
							blocks.swap(last + l, n - l);
					}
			}
			right = (n- right_cache)as isize;
		}



    blocks
        .into_iter()
        .enumerate()
        .filter_map(|(i, opt)| opt.map(|v| v * i))
        .sum()
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
