use std::io::{self, BufRead};

fn solve_puzzle(input: &str) -> u64 {
    let mut total: u64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let digits: Vec<char> = line.chars().collect();
        let n = digits.len();

        // We need to pick exactly 12 digits to maximize the resulting number
        // Greedy approach: at each step, pick the largest digit that still allows
        // us to pick enough remaining digits

        let mut result = String::new();
        let mut start = 0; // Current starting position to search from
        let mut remaining = 12; // How many digits we still need to pick

        while remaining > 0 {
            // We need to pick 'remaining' digits from digits[start..]
            // For the next digit, we can search up to position (n - remaining)
            // because we need to leave at least (remaining - 1) digits after this pick
            let end = n - remaining + 1;

            // Find the maximum digit in the range [start, end)
            let mut max_digit = '0';
            let mut max_pos = start;

            for i in start..end {
                if digits[i] > max_digit {
                    max_digit = digits[i];
                    max_pos = i;
                }
            }

            result.push(max_digit);
            start = max_pos + 1; // Next search starts after the selected position
            remaining -= 1;
        }

        // Parse the 12-digit number
        let joltage: u64 = result.parse().unwrap();
        total += joltage;
    }

    total
}

fn main() {
    let stdin = io::stdin();
    let input: String = stdin.lock().lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let result = solve_puzzle(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(solve_puzzle(input), 3121910778619);
    }

    #[test]
    fn test_line1() {
        // 987654321111111 -> 987654321111 (first 12 digits)
        let input = "987654321111111";
        assert_eq!(solve_puzzle(input), 987654321111);
    }

    #[test]
    fn test_line2() {
        // 811111111111119 -> 811111111119 (all except some 1s, keep the 9 at end)
        let input = "811111111111119";
        assert_eq!(solve_puzzle(input), 811111111119);
    }

    #[test]
    fn test_line3() {
        // 234234234234278 -> 434234234278 (skip 2, 3, 2 at start)
        let input = "234234234234278";
        assert_eq!(solve_puzzle(input), 434234234278);
    }

    #[test]
    fn test_line4() {
        // 818181911112111 -> 888911112111 (skip some 1s at front)
        let input = "818181911112111";
        assert_eq!(solve_puzzle(input), 888911112111);
    }
}
