// Advent of Code 2025 - Day 1, Part 1
// Secret Entrance - Safe Dial Puzzle

fn solve_puzzle(input: &str) -> u32 {
    let mut position: i32 = 50; // Dial starts at 50
    let mut count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = &line[0..1];
        let distance: i32 = line[1..].parse().unwrap();

        match direction {
            "L" => position -= distance,
            "R" => position += distance,
            _ => panic!("Unknown direction: {}", direction),
        }

        // Normalize position to 0-99 range
        position = position.rem_euclid(100);

        // Check if dial ends at 0
        if position == 0 {
            count += 1;
        }
    }

    count
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
    fn test_example_level1() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(solve_puzzle(input), 3);
    }
}
