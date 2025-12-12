use std::collections::HashMap;
use std::io::{self, BufRead};

fn solve_puzzle(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }

    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    // Find starting position 'S'
    let mut start_col = 0;
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == 'S' {
                start_col = col;
                break;
            }
        }
    }

    // Track timeline counts at each column position
    // Key: column, Value: number of timelines at that position
    let mut timeline_counts: HashMap<usize, u64> = HashMap::new();
    timeline_counts.insert(start_col, 1);

    // Process each row
    for row in 0..rows {
        if timeline_counts.is_empty() {
            break;
        }

        // Find splitters hit by timelines in this row
        let mut splitters_hit: Vec<(usize, u64)> = Vec::new();
        for (&col, &count) in &timeline_counts {
            if col < cols && grid[row][col] == '^' {
                splitters_hit.push((col, count));
            }
        }

        // If any splitters were hit, process them
        if !splitters_hit.is_empty() {
            let mut new_counts: HashMap<usize, u64> = HashMap::new();

            // Keep timelines that didn't hit splitters
            for (&col, &count) in &timeline_counts {
                if col < cols && grid[row][col] != '^' {
                    *new_counts.entry(col).or_insert(0) += count;
                }
            }

            // Add new timelines from splitters (each timeline splits into 2)
            for (splitter_col, count) in &splitters_hit {
                // Left path: each timeline at this splitter becomes a timeline going left
                if *splitter_col > 0 {
                    *new_counts.entry(splitter_col - 1).or_insert(0) += count;
                }
                // Right path: each timeline at this splitter becomes a timeline going right
                if splitter_col + 1 < cols {
                    *new_counts.entry(splitter_col + 1).or_insert(0) += count;
                }
            }

            timeline_counts = new_counts;
        }
    }

    // Sum up all timelines at the end
    timeline_counts.values().sum()
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
    fn test_example1() {
        let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";
        assert_eq!(solve_puzzle(input), 40);
    }
}
