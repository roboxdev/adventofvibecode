use std::io::{self, BufRead};

fn solve_puzzle(input: &str) -> u64 {
    let mut parts = input.trim().split("\n\n");

    // Parse the ranges
    let ranges_str = parts.next().unwrap_or("");
    let ranges: Vec<(u64, u64)> = ranges_str
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut nums = line.split('-');
            let start: u64 = nums.next().unwrap().parse().unwrap();
            let end: u64 = nums.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect();

    // Parse the ingredient IDs
    let ids_str = parts.next().unwrap_or("");
    let ids: Vec<u64> = ids_str
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    // Count how many IDs are fresh (fall within any range)
    let mut count = 0;
    for id in ids {
        let is_fresh = ranges.iter().any(|&(start, end)| id >= start && id <= end);
        if is_fresh {
            count += 1;
        }
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
    fn test_example_1() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        assert_eq!(solve_puzzle(input), 3);
    }
}
