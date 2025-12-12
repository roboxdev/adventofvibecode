use std::io::{self, Read};

fn parse_line(line: &str) -> (Vec<Vec<usize>>, Vec<i64>) {
    let mut buttons = Vec::new();
    let mut in_paren = false;
    let mut current = String::new();

    for c in line.chars() {
        if c == '(' {
            in_paren = true;
            current.clear();
        } else if c == ')' {
            in_paren = false;
            let indices: Vec<usize> = current
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            buttons.push(indices);
        } else if in_paren {
            current.push(c);
        }
    }

    let mut targets = Vec::new();
    if let Some(start) = line.find('{') {
        if let Some(end) = line.find('}') {
            let content = &line[start+1..end];
            targets = content
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
        }
    }

    (buttons, targets)
}

// GCD for rational arithmetic
fn gcd(a: i64, b: i64) -> i64 {
    let a = a.abs();
    let b = b.abs();
    if b == 0 { a } else { gcd(b, a % b) }
}

// Rational number for exact arithmetic
#[derive(Clone, Copy, Debug)]
struct Rational {
    num: i64,
    den: i64,
}

impl Rational {
    fn new(num: i64, den: i64) -> Self {
        if den == 0 {
            panic!("Division by zero");
        }
        let g = gcd(num, den);
        let (num, den) = if den < 0 {
            (-num / g, -den / g)
        } else {
            (num / g, den / g)
        };
        Rational { num, den }
    }

    fn zero() -> Self {
        Rational { num: 0, den: 1 }
    }

    fn from_int(n: i64) -> Self {
        Rational { num: n, den: 1 }
    }

    fn is_zero(&self) -> bool {
        self.num == 0
    }

    fn is_integer(&self) -> bool {
        self.den == 1
    }

    fn to_int(&self) -> i64 {
        assert!(self.is_integer(), "Not an integer: {}/{}", self.num, self.den);
        self.num
    }

    fn floor(&self) -> i64 {
        if self.num >= 0 {
            self.num / self.den
        } else {
            (self.num - self.den + 1) / self.den
        }
    }

    fn ceil(&self) -> i64 {
        if self.num >= 0 {
            (self.num + self.den - 1) / self.den
        } else {
            self.num / self.den
        }
    }
}

impl std::ops::Add for Rational {
    type Output = Rational;
    fn add(self, other: Rational) -> Rational {
        Rational::new(self.num * other.den + other.num * self.den, self.den * other.den)
    }
}

impl std::ops::Sub for Rational {
    type Output = Rational;
    fn sub(self, other: Rational) -> Rational {
        Rational::new(self.num * other.den - other.num * self.den, self.den * other.den)
    }
}

impl std::ops::Mul for Rational {
    type Output = Rational;
    fn mul(self, other: Rational) -> Rational {
        Rational::new(self.num * other.num, self.den * other.den)
    }
}

impl std::ops::Div for Rational {
    type Output = Rational;
    fn div(self, other: Rational) -> Rational {
        Rational::new(self.num * other.den, self.den * other.num)
    }
}

impl std::ops::Neg for Rational {
    type Output = Rational;
    fn neg(self) -> Rational {
        Rational { num: -self.num, den: self.den }
    }
}

// Solve using Gaussian elimination to reduce the problem
// Then enumerate over free variables
fn solve_machine(buttons: &[Vec<usize>], targets: &[i64]) -> i64 {
    let n = targets.len();
    let m = buttons.len();

    if n == 0 || targets.iter().all(|&x| x == 0) {
        return 0;
    }

    // Build the augmented matrix [A | b] where A is n x m
    // Row j corresponds to counter j
    // Column i corresponds to button i
    let mut matrix: Vec<Vec<Rational>> = vec![vec![Rational::zero(); m + 1]; n];
    for i in 0..m {
        for &idx in &buttons[i] {
            if idx < n {
                matrix[idx][i] = Rational::from_int(1);
            }
        }
    }
    for j in 0..n {
        matrix[j][m] = Rational::from_int(targets[j]);
    }

    // Gaussian elimination to reduced row echelon form
    let mut pivot_row = 0;
    let mut pivot_cols: Vec<usize> = Vec::new();
    let mut free_cols: Vec<usize> = Vec::new();

    for col in 0..m {
        // Find pivot in this column
        let mut found = None;
        for row in pivot_row..n {
            if !matrix[row][col].is_zero() {
                found = Some(row);
                break;
            }
        }

        if let Some(row) = found {
            // Swap rows
            matrix.swap(pivot_row, row);
            pivot_cols.push(col);

            // Scale pivot row to have leading 1
            let pivot_val = matrix[pivot_row][col];
            for c in 0..=m {
                matrix[pivot_row][c] = matrix[pivot_row][c] / pivot_val;
            }

            // Eliminate column in all other rows
            for r in 0..n {
                if r != pivot_row && !matrix[r][col].is_zero() {
                    let factor = matrix[r][col];
                    for c in 0..=m {
                        let sub = factor * matrix[pivot_row][c];
                        matrix[r][c] = matrix[r][c] - sub;
                    }
                }
            }

            pivot_row += 1;
        } else {
            free_cols.push(col);
        }
    }

    // Check for inconsistency: rows with all zeros but non-zero RHS
    for r in pivot_row..n {
        if !matrix[r][m].is_zero() {
            return i64::MAX / 2; // No solution
        }
    }

    // Now we have reduced row echelon form
    // pivot_cols are basic variables, free_cols are free variables
    // For each assignment of free variables, basic variables are determined

    let num_pivots = pivot_cols.len();

    // For each pivot row r (corresponding to pivot column pivot_cols[r]):
    // x_{pivot_cols[r]} = matrix[r][m] - sum_{f in free_cols} matrix[r][f] * x_f

    // We need all x_i >= 0 (non-negative)
    // And we want to minimize sum x_i

    let max_target = *targets.iter().max().unwrap_or(&0);

    // Enumerate all combinations of free variable values
    let mut best = i64::MAX / 2;

    fn enumerate(
        free_idx: usize,
        free_values: &mut Vec<i64>,
        free_cols: &[usize],
        matrix: &[Vec<Rational>],
        num_pivots: usize,
        m: usize,
        max_val: i64,
        best: &mut i64,
        current_sum: i64,
    ) {
        let num_free = free_cols.len();

        if free_idx == num_free {
            // All free variables assigned, compute basic variables
            let mut total = current_sum;
            let mut valid = true;

            // Compute basic variables
            for r in 0..num_pivots {
                let mut val = matrix[r][m];
                for (f_idx, &f_col) in free_cols.iter().enumerate() {
                    val = val - matrix[r][f_col] * Rational::from_int(free_values[f_idx]);
                }

                if !val.is_integer() || val.num < 0 {
                    valid = false;
                    break;
                }
                total += val.to_int();
            }

            if valid && total < *best {
                *best = total;
            }
            return;
        }

        // Pruning: if current sum already exceeds best, skip
        if current_sum >= *best {
            return;
        }

        // Compute bounds for this free variable, taking into account
        // the maximum contribution that future free variables can provide
        let f_col = free_cols[free_idx];
        let mut lower_bound: i64 = 0;
        let mut upper_bound: i64 = max_val;

        for r in 0..num_pivots {
            // Compute remaining RHS after accounting for already-assigned free variables
            let mut rhs = matrix[r][m];
            for f_idx in 0..free_idx {
                rhs = rhs - matrix[r][free_cols[f_idx]] * Rational::from_int(free_values[f_idx]);
            }

            // For future free variables, compute the maximum positive contribution they can make
            // and the maximum negative contribution they can make
            let mut max_future_positive = Rational::zero();
            let mut max_future_negative = Rational::zero();

            for f_idx in (free_idx + 1)..num_free {
                let future_coef = matrix[r][free_cols[f_idx]];
                if future_coef.num > 0 {
                    // This future variable could decrease the basic variable by up to coef * max_val
                    // But if we set it to 0, it contributes 0
                    max_future_negative = max_future_negative - future_coef * Rational::from_int(max_val);
                } else if future_coef.num < 0 {
                    // This future variable could increase the basic variable by up to -coef * max_val
                    max_future_positive = max_future_positive - future_coef * Rational::from_int(max_val);
                }
            }

            // Constraint: rhs - coef * x_f + [future contribution] >= 0
            // The future contribution ranges from max_future_negative to max_future_positive

            let coef = matrix[r][f_col];
            if coef.num > 0 {
                // To get a valid upper bound, assume future contributes max (max_future_positive)
                // rhs - coef * x_f + max_future_positive >= 0
                // => x_f <= (rhs + max_future_positive) / coef
                let bound = (rhs + max_future_positive) / coef;
                let bound_floor = bound.floor();
                upper_bound = upper_bound.min(bound_floor);
            } else if coef.num < 0 {
                // To get a valid lower bound, assume future contributes max (max_future_positive)
                // rhs - coef * x_f + max_future_positive >= 0
                // coef < 0, so -coef * x_f <= rhs + max_future_positive
                // x_f >= (rhs + max_future_positive) / coef  (since coef < 0, dividing flips)
                let bound = (rhs + max_future_positive) / coef;
                let bound_ceil = bound.ceil();
                lower_bound = lower_bound.max(bound_ceil);
            }
        }

        if lower_bound > upper_bound || upper_bound < 0 {
            return;
        }
        lower_bound = lower_bound.max(0);

        for val in lower_bound..=upper_bound {
            free_values.push(val);
            enumerate(
                free_idx + 1,
                free_values,
                free_cols,
                matrix,
                num_pivots,
                m,
                max_val,
                best,
                current_sum + val,
            );
            free_values.pop();
        }
    }

    let mut free_values = Vec::new();
    enumerate(
        0,
        &mut free_values,
        &free_cols,
        &matrix,
        num_pivots,
        m,
        max_target,
        &mut best,
        0,
    );

    best
}

fn solve_puzzle(input: &str) -> i64 {
    let mut total = 0i64;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (buttons, targets) = parse_line(line);
        let result = solve_machine(&buttons, &targets);
        if result >= i64::MAX / 2 {
            panic!("No solution found for line: {}", line);
        }
        total += result;
    }

    total
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", solve_puzzle(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        assert_eq!(solve_puzzle(input), 10);
    }

    #[test]
    fn test_example2() {
        let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        assert_eq!(solve_puzzle(input), 12);
    }

    #[test]
    fn test_example3() {
        let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(solve_puzzle(input), 11);
    }

    #[test]
    fn test_all_examples() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(solve_puzzle(input), 33);
    }

    #[test]
    fn test_problem_line() {
        let input = "[.###.####.] (0,1,2,3,4,5,7,8,9) (4,9) (1,2,5,7,9) (0,1,2,3,5,6,7,8,9) (0,2,4,6,8,9) (2,3,4,6,9) (3,7,9) (8) (3,8) (0,1,4,5,6,7,8,9) (1,5,8) (0,1,3,4,5,8,9) {37,40,38,45,42,40,32,32,63,72}";
        let result = solve_puzzle(input);
        assert!(result < i64::MAX / 2, "Should find a solution, got: {}", result);
    }
}
