// Advent of Code 2025 Day 1 Part 2
// Count the number of times the dial points at 0 during ANY click

fn solve_puzzle(input: &str) -> u64 {
    let mut position: i64 = 50;
    let mut zero_count: u64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = &line[0..1];
        let distance: i64 = line[1..].parse().expect("Invalid distance");

        // Determine the movement direction
        let step: i64 = if direction == "R" { 1 } else { -1 };

        // Process each click
        for _ in 0..distance {
            position = (position + step).rem_euclid(100);
            if position == 0 {
                zero_count += 1;
            }
        }
    }

    zero_count
}

fn main() {
    let input = include_str!("input.md");
    let answer = solve_puzzle(input);
    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_level2() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(solve_puzzle(input), 6);
    }
}
