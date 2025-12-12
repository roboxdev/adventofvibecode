use std::io::{self, BufRead};

fn count_adjacent_rolls(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Check all 8 adjacent positions
    for dr in -1i32..=1 {
        for dc in -1i32..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                if grid[nr as usize][nc as usize] == '@' {
                    count += 1;
                }
            }
        }
    }
    count
}

fn find_accessible_rolls(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut accessible = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' {
                let adjacent = count_adjacent_rolls(grid, r, c);
                // Accessible if fewer than 4 adjacent rolls
                if adjacent < 4 {
                    accessible.push((r, c));
                }
            }
        }
    }
    accessible
}

fn solve_puzzle(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    if grid.is_empty() {
        return 0;
    }

    let mut total_removed = 0;

    loop {
        let accessible = find_accessible_rolls(&grid);
        if accessible.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (r, c) in &accessible {
            grid[*r][*c] = '.';
        }
        total_removed += accessible.len();
    }

    total_removed
}

fn main() {
    let stdin = io::stdin();
    let input: String = stdin.lock().lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>()
        .join("\n");

    let result = solve_puzzle(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        assert_eq!(solve_puzzle(input), 43);
    }
}
