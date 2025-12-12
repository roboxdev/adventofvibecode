use std::io::{self, BufRead};

fn solve_puzzle(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }

    // Find the maximum width
    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Pad all lines to the same length
    let padded_lines: Vec<String> = lines.iter()
        .map(|l| format!("{:width$}", l, width = max_width))
        .collect();

    // The last line contains the operators
    let operator_line = &padded_lines[padded_lines.len() - 1];
    let number_lines = &padded_lines[..padded_lines.len() - 1];

    // Identify problem boundaries by finding columns
    // A problem column contains at least one digit in the number rows or an operator in the last row
    // Problem columns are separated by columns that are entirely spaces (including the operator row)

    // First, identify which columns belong to which problem
    // A column is a separator if it's a space in all rows (including operator row)

    let mut column_is_separator = vec![true; max_width];

    for col in 0..max_width {
        for line in padded_lines.iter() {
            let ch = line.chars().nth(col).unwrap_or(' ');
            if ch != ' ' {
                column_is_separator[col] = false;
                break;
            }
        }
    }

    // Group consecutive non-separator columns into problems
    let mut problems: Vec<(usize, usize)> = Vec::new(); // (start_col, end_col)
    let mut in_problem = false;
    let mut start = 0;

    for col in 0..max_width {
        if !column_is_separator[col] {
            if !in_problem {
                start = col;
                in_problem = true;
            }
        } else {
            if in_problem {
                problems.push((start, col));
                in_problem = false;
            }
        }
    }
    if in_problem {
        problems.push((start, max_width));
    }

    let mut grand_total: u64 = 0;

    for (start_col, end_col) in problems {
        // Extract the operator from the operator line for this problem
        let op_slice: String = operator_line.chars().skip(start_col).take(end_col - start_col).collect();
        let operator = if op_slice.contains('*') {
            '*'
        } else if op_slice.contains('+') {
            '+'
        } else {
            continue; // No valid operator found
        };

        // Extract numbers from this problem column range
        let mut numbers: Vec<u64> = Vec::new();

        for line in number_lines.iter() {
            let slice: String = line.chars().skip(start_col).take(end_col - start_col).collect();
            // Parse numbers from this slice
            let num_str: String = slice.chars().filter(|c| c.is_ascii_digit()).collect();
            if !num_str.is_empty() {
                if let Ok(n) = num_str.parse::<u64>() {
                    numbers.push(n);
                }
            }
        }

        // Compute the result based on the operator
        let result: u64 = if operator == '+' {
            numbers.iter().sum()
        } else {
            numbers.iter().product()
        };

        grand_total += result;
    }

    grand_total
}

fn main() {
    let stdin = io::stdin();
    let input: String = stdin.lock().lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let result = solve_puzzle(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        assert_eq!(solve_puzzle(input), 4277556);
    }
}
