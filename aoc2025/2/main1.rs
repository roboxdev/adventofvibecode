use std::io::{self, BufRead};

/// Check if a number is "invalid" - meaning it's made of some sequence of digits repeated twice
/// For example: 11 (1 repeated), 6464 (64 repeated), 123123 (123 repeated)
fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Must have even number of digits to be a repeated pattern
    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    let first_half = &s[..half];
    let second_half = &s[half..];

    // Check if the first half equals the second half
    // Also ensure no leading zero in the repeated pattern (the number itself wouldn't have leading zeros)
    first_half == second_half && !first_half.starts_with('0')
}

fn solve_puzzle(input: &str) -> u64 {
    let input = input.trim();
    let mut sum: u64 = 0;

    for range_str in input.split(',') {
        let range_str = range_str.trim();
        if range_str.is_empty() {
            continue;
        }

        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            continue;
        }

        let start: u64 = parts[0].trim().parse().expect("Invalid start number");
        let end: u64 = parts[1].trim().parse().expect("Invalid end number");

        // Instead of iterating through all numbers, we can be smarter
        // Generate all possible invalid IDs that could fall in this range
        // An invalid ID has even number of digits and is formed by repeating a half

        // Determine the range of digit lengths to consider
        let min_digits = start.to_string().len();
        let max_digits = end.to_string().len();

        for digit_len in min_digits..=max_digits {
            if digit_len % 2 != 0 {
                continue;
            }

            let half_len = digit_len / 2;

            // Generate all possible "halves" that could form invalid IDs in range
            // The half must have half_len digits and no leading zeros
            let half_min = if half_len == 1 { 1 } else { 10u64.pow((half_len - 1) as u32) };
            let half_max = 10u64.pow(half_len as u32) - 1;

            for half in half_min..=half_max {
                // Form the invalid ID by repeating the half
                let half_str = half.to_string();
                let invalid_id_str = format!("{}{}", half_str, half_str);
                let invalid_id: u64 = invalid_id_str.parse().unwrap();

                if invalid_id >= start && invalid_id <= end {
                    sum += invalid_id;
                }
            }
        }
    }

    sum
}

fn main() {
    let stdin = io::stdin();
    let input: String = stdin.lock().lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>()
        .join("\n");

    let result = solve_puzzle(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_id() {
        assert!(is_invalid_id(11));
        assert!(is_invalid_id(22));
        assert!(is_invalid_id(99));
        assert!(is_invalid_id(1010));
        assert!(is_invalid_id(6464));
        assert!(is_invalid_id(123123));
        assert!(is_invalid_id(222222));
        assert!(is_invalid_id(446446));
        assert!(is_invalid_id(38593859));
        assert!(is_invalid_id(1188511885));

        assert!(!is_invalid_id(101)); // odd digits
        assert!(!is_invalid_id(12));  // 1 != 2
        assert!(!is_invalid_id(1234)); // 12 != 34
    }

    #[test]
    fn test_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(solve_puzzle(input), 1227775554);
    }
}
