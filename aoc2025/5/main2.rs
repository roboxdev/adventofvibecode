use std::io::{self, BufRead};

fn solve_puzzle(input: &str) -> u64 {
    // Parse the ranges (only the first section, before blank line or end)
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            // Stop at blank line (Part 2 ignores the second section)
            break;
        }

        if let Some((start_str, end_str)) = line.split_once('-') {
            if let (Ok(start), Ok(end)) = (start_str.parse::<u64>(), end_str.parse::<u64>()) {
                ranges.push((start, end));
            }
        }
    }

    if ranges.is_empty() {
        return 0;
    }

    // Sort ranges by start
    ranges.sort_by_key(|r| r.0);

    // Merge overlapping ranges and count total fresh IDs
    let mut merged: Vec<(u64, u64)> = Vec::new();

    for (start, end) in ranges {
        if merged.is_empty() {
            merged.push((start, end));
        } else {
            let last = merged.last_mut().unwrap();
            // Check if current range overlaps or is adjacent to the last merged range
            if start <= last.1 + 1 {
                // Extend the last range if needed
                last.1 = last.1.max(end);
            } else {
                // No overlap, start a new range
                merged.push((start, end));
            }
        }
    }

    // Count total fresh IDs
    let mut count = 0u64;
    for (start, end) in merged {
        count += end - start + 1;
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let input: String = stdin.lock().lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    let result = solve_puzzle(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_level2() {
        let input = "3-5\n10-14\n16-20\n12-18";
        assert_eq!(solve_puzzle(input), 14);
    }
}
