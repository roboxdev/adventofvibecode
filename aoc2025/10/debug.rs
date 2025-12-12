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

fn gcd(a: i64, b: i64) -> i64 {
    let a = a.abs();
    let b = b.abs();
    if b == 0 { a } else { gcd(b, a % b) }
}

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
        self.num
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

impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

fn main() {
    let line = "[.###.####.] (0,1,2,3,4,5,7,8,9) (4,9) (1,2,5,7,9) (0,1,2,3,5,6,7,8,9) (0,2,4,6,8,9) (2,3,4,6,9) (3,7,9) (8) (3,8) (0,1,4,5,6,7,8,9) (1,5,8) (0,1,3,4,5,8,9) {37,40,38,45,42,40,32,32,63,72}";

    let (buttons, targets) = parse_line(line);
    let n = targets.len();
    let m = buttons.len();

    println!("Buttons: {}", m);
    println!("Counters: {}", n);
    println!("Targets: {:?}", targets);

    // Build the augmented matrix [A | b] where A is n x m
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

    println!("\nOriginal matrix:");
    for r in 0..n {
        for c in 0..=m {
            print!("{:>4} ", matrix[r][c]);
        }
        println!();
    }

    // Gaussian elimination
    let mut pivot_row = 0;
    let mut pivot_cols: Vec<usize> = Vec::new();
    let mut free_cols: Vec<usize> = Vec::new();

    for col in 0..m {
        let mut found = None;
        for row in pivot_row..n {
            if !matrix[row][col].is_zero() {
                found = Some(row);
                break;
            }
        }

        if let Some(row) = found {
            matrix.swap(pivot_row, row);
            pivot_cols.push(col);

            let pivot_val = matrix[pivot_row][col];
            for c in 0..=m {
                matrix[pivot_row][c] = matrix[pivot_row][c] / pivot_val;
            }

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

    println!("\nReduced matrix:");
    for r in 0..n {
        for c in 0..=m {
            print!("{:>6} ", matrix[r][c]);
        }
        println!();
    }

    println!("\nPivot columns: {:?}", pivot_cols);
    println!("Free columns: {:?}", free_cols);
    println!("Number of pivots: {}", pivot_cols.len());
    println!("Number of free variables: {}", free_cols.len());

    // Check for inconsistency
    for r in pivot_row..n {
        if !matrix[r][m].is_zero() {
            println!("INCONSISTENT - row {} has zeros but non-zero RHS: {}", r, matrix[r][m]);
        }
    }
}
