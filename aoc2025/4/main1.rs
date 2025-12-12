use std::io::{self, BufRead};

fn solve_puzzle(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // 8 directions: up, down, left, right, and 4 diagonals
    let directions: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    for row in 0..rows {
        for col in 0..cols {
            // Only check positions that have a paper roll
            if grid[row][col] == '@' {
                // Count adjacent paper rolls
                let mut adjacent_rolls = 0;

                for (dr, dc) in &directions {
                    let new_row = row as i32 + dr;
                    let new_col = col as i32 + dc;

                    if new_row >= 0 && new_row < rows as i32
                       && new_col >= 0 && new_col < cols as i32 {
                        if grid[new_row as usize][new_col as usize] == '@' {
                            adjacent_rolls += 1;
                        }
                    }
                }

                // Forklift can access if fewer than 4 adjacent rolls
                if adjacent_rolls < 4 {
                    count += 1;
                }
            }
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
    fn test_example1() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        assert_eq!(solve_puzzle(input), 13);
    }
}
