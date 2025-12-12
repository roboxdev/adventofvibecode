use std::collections::HashSet;
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

    // Track active beam positions (columns where beams are going downward)
    let mut beam_positions: HashSet<usize> = HashSet::new();
    beam_positions.insert(start_col);

    let mut split_count: u64 = 0;

    // Process each row starting from the row after S
    for row in 0..rows {
        if beam_positions.is_empty() {
            break;
        }

        // Find splitters hit by beams in this row
        let mut splitters_hit: Vec<usize> = Vec::new();
        for &col in &beam_positions {
            if col < cols && grid[row][col] == '^' {
                splitters_hit.push(col);
            }
        }

        // If any splitters were hit, process them
        if !splitters_hit.is_empty() {
            // Count splits
            split_count += splitters_hit.len() as u64;

            // Remove beams that hit splitters and add new beams from left/right
            let mut new_positions: HashSet<usize> = HashSet::new();

            // Keep beams that didn't hit splitters
            for &col in &beam_positions {
                if col < cols && grid[row][col] != '^' {
                    new_positions.insert(col);
                }
            }

            // Add new beams from splitters (left and right)
            for &splitter_col in &splitters_hit {
                if splitter_col > 0 {
                    new_positions.insert(splitter_col - 1);
                }
                if splitter_col + 1 < cols {
                    new_positions.insert(splitter_col + 1);
                }
            }

            beam_positions = new_positions;
        }
    }

    split_count
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
        assert_eq!(solve_puzzle(input), 21);
    }
}
