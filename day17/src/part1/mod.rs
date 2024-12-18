fn combo(operand: &u32, a: &u32, b: &u32, c: &u32) -> u32 {
    match operand {
        4 => return *a,
        5 => return *b,
        6 => return *c,
        n => return *n,
    }
}

#[allow(unused)]
pub fn part1() -> String {
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();
    let mut a: u32 = input[0][12..].parse().unwrap();
    let mut b: u32 = input[1][12..].parse().unwrap();
    let mut c: u32 = input[2][12..].parse().unwrap();
    let program: Vec<u32> = input[4][9..]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut ans = Vec::new();
    let mut pointer = 0;
    while pointer <= program.len() - 2 {
        let opcode = program[pointer];
        let operand = program[pointer + 1];
        match opcode {
            0 => {
                //adv
                a /= 2u32.pow(combo(&operand, &a, &b, &c));
            }
            1 => {
                // bxl
                b ^= operand;
            }
            2 => {
                //bst
                b = combo(&operand, &a, &b, &c) % 8;
            }
            3 => {
                // jnz
                if a != 0 {
                    pointer = operand as usize;
                    continue;
                }
            }
            4 => {
                // bxl
                b ^= c;
            }
            5 => {
                // out
                ans.push((combo(&operand, &a, &b, &c) % 8).to_string());
            }
            6 => {
                // bdv
                b = a / 2u32.pow(combo(&operand, &a, &b, &c));
            }
            7 => {
                // cdv
                c = a / 2u32.pow(combo(&operand, &a, &b, &c));
            }
            _ => panic!("opcode not found"),
        }
        pointer += 2;
    }
    ans.join(",")
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
