use std::io::{self, BufRead};

/// Check if a number is invalid (made of a sequence repeated at least twice)
fn is_invalid(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Try all possible pattern lengths from 1 to len/2
    // (pattern must repeat at least twice, so max pattern length is len/2)
    for pattern_len in 1..=len / 2 {
        // The total length must be divisible by the pattern length
        if len % pattern_len == 0 {
            let pattern = &s[0..pattern_len];
            let repetitions = len / pattern_len;

            // Pattern must repeat at least twice
            if repetitions >= 2 {
                // Check if the entire string is made of this pattern
                let repeated: String = pattern.repeat(repetitions);
                if repeated == s {
                    return true;
                }
            }
        }
    }

    false
}

fn solve_puzzle(input: &str) -> u64 {
    let mut sum = 0u64;

    // Parse ranges - handle potential whitespace and newlines
    let cleaned = input.trim().replace('\n', "").replace(' ', "");

    for range_str in cleaned.split(',') {
        if range_str.is_empty() {
            continue;
        }

        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            continue;
        }

        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();

        for n in start..=end {
            if is_invalid(n) {
                sum += n;
            }
        }
    }

    sum
}

fn main() {
    let stdin = io::stdin();
    let input: String = stdin.lock().lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<_>>()
        .join("\n");

    let result = solve_puzzle(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid() {
        // Two repetitions
        assert!(is_invalid(11));
        assert!(is_invalid(22));
        assert!(is_invalid(99));
        assert!(is_invalid(1010));
        assert!(is_invalid(1188511885));
        assert!(is_invalid(222222)); // "222" repeated 2 times or "22" repeated 3 times, etc.
        assert!(is_invalid(446446));
        assert!(is_invalid(38593859));

        // Three or more repetitions (new for Part 2)
        assert!(is_invalid(111)); // "1" repeated 3 times
        assert!(is_invalid(999)); // "9" repeated 3 times
        assert!(is_invalid(565656)); // "56" repeated 3 times
        assert!(is_invalid(824824824)); // "824" repeated 3 times
        assert!(is_invalid(2121212121)); // "21" repeated 5 times
        assert!(is_invalid(123123123)); // "123" repeated 3 times
        assert!(is_invalid(1111111)); // "1" repeated 7 times
        assert!(is_invalid(1212121212)); // "12" repeated 5 times

        // Valid IDs
        assert!(!is_invalid(101));
        assert!(!is_invalid(12345));
        assert!(!is_invalid(1698522));
    }

    #[test]
    fn test_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(solve_puzzle(input), 4174379265);
    }
}
