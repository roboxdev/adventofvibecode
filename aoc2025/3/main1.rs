use std::io::{self, BufRead};

fn solve_puzzle(input: &str) -> u64 {
    let mut total: u64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let digits: Vec<u32> = line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        let mut max_joltage: u32 = 0;

        // Try all pairs (i, j) where i < j
        // The joltage is digits[i] * 10 + digits[j]
        for i in 0..digits.len() {
            for j in (i + 1)..digits.len() {
                let joltage = digits[i] * 10 + digits[j];
                if joltage > max_joltage {
                    max_joltage = joltage;
                }
            }
        }

        total += max_joltage as u64;
    }

    total
}

fn main() {
    let stdin = io::stdin();
    let input: String = stdin.lock().lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>()
        .join("\n");

    let answer = solve_puzzle(&input);
    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(solve_puzzle(input), 357);
    }

    #[test]
    fn test_single_line_1() {
        // 987654321111111 -> max is 98 (first two digits)
        let input = "987654321111111";
        assert_eq!(solve_puzzle(input), 98);
    }

    #[test]
    fn test_single_line_2() {
        // 811111111111119 -> max is 89 (8 at start, 9 at end)
        let input = "811111111111119";
        assert_eq!(solve_puzzle(input), 89);
    }

    #[test]
    fn test_single_line_3() {
        // 234234234234278 -> max is 78 (last two digits)
        let input = "234234234234278";
        assert_eq!(solve_puzzle(input), 78);
    }

    #[test]
    fn test_single_line_4() {
        // 818181911112111 -> max is 92 (9 followed by 2)
        let input = "818181911112111";
        assert_eq!(solve_puzzle(input), 92);
    }
}
