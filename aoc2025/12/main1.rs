use std::collections::HashSet;
use std::io::{self, BufRead};

type Shape = Vec<(i32, i32)>;

fn parse_shape(lines: &[&str]) -> Shape {
    let mut cells = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                cells.push((x as i32, y as i32));
            }
        }
    }
    normalize(&cells)
}

fn normalize(cells: &[(i32, i32)]) -> Shape {
    if cells.is_empty() {
        return Vec::new();
    }
    let min_x = cells.iter().map(|c| c.0).min().unwrap();
    let min_y = cells.iter().map(|c| c.1).min().unwrap();
    let mut result: Vec<(i32, i32)> = cells.iter().map(|c| (c.0 - min_x, c.1 - min_y)).collect();
    result.sort();
    result
}

fn rotate_90(shape: &Shape) -> Shape {
    // (x, y) -> (-y, x)
    let rotated: Vec<(i32, i32)> = shape.iter().map(|&(x, y)| (-y, x)).collect();
    normalize(&rotated)
}

fn flip_h(shape: &Shape) -> Shape {
    // (x, y) -> (-x, y)
    let flipped: Vec<(i32, i32)> = shape.iter().map(|&(x, y)| (-x, y)).collect();
    normalize(&flipped)
}

fn all_orientations(shape: &Shape) -> Vec<Shape> {
    let mut orientations = HashSet::new();
    let mut current = shape.clone();

    for _ in 0..4 {
        orientations.insert(current.clone());
        orientations.insert(flip_h(&current));
        current = rotate_90(&current);
    }

    orientations.into_iter().collect()
}

fn can_place(grid: &[Vec<bool>], shape: &Shape, ox: i32, oy: i32, width: usize, height: usize) -> bool {
    for &(dx, dy) in shape {
        let nx = ox + dx;
        let ny = oy + dy;
        if nx < 0 || ny < 0 || nx >= width as i32 || ny >= height as i32 {
            return false;
        }
        if grid[ny as usize][nx as usize] {
            return false;
        }
    }
    true
}

fn place(grid: &mut [Vec<bool>], shape: &Shape, ox: i32, oy: i32) {
    for &(dx, dy) in shape {
        grid[(oy + dy) as usize][(ox + dx) as usize] = true;
    }
}

fn unplace(grid: &mut [Vec<bool>], shape: &Shape, ox: i32, oy: i32) {
    for &(dx, dy) in shape {
        grid[(oy + dy) as usize][(ox + dx) as usize] = false;
    }
}

// Solve: try to place all pieces in the grid
// The grid doesn't have to be fully covered
fn solve(
    grid: &mut Vec<Vec<bool>>,
    pieces: &[usize], // indices into all_shapes (remaining pieces to place)
    piece_idx: usize, // current piece index in pieces array
    all_shapes: &[Vec<Shape>], // all_shapes[shape_idx] = list of orientations
    width: usize,
    height: usize,
) -> bool {
    // All pieces placed successfully
    if piece_idx >= pieces.len() {
        return true;
    }

    let shape_idx = pieces[piece_idx];

    // Try each orientation
    for orientation in &all_shapes[shape_idx] {
        // Try placing at each position
        for y in 0..height as i32 {
            for x in 0..width as i32 {
                if can_place(grid, orientation, x, y, width, height) {
                    place(grid, orientation, x, y);
                    if solve(grid, pieces, piece_idx + 1, all_shapes, width, height) {
                        return true;
                    }
                    unplace(grid, orientation, x, y);
                }
            }
        }
    }

    false
}

fn can_fit_all(shapes: &[Shape], width: usize, height: usize, quantities: &[usize]) -> bool {
    // Count total cells needed
    let total_cells: usize = shapes.iter().zip(quantities.iter())
        .map(|(s, &q)| s.len() * q)
        .sum();

    let grid_cells = width * height;

    // If we need more cells than available, impossible
    if total_cells > grid_cells {
        return false;
    }

    // If no pieces needed, always possible
    if total_cells == 0 {
        return true;
    }

    // Precompute all orientations for each shape
    let all_orientations_vec: Vec<Vec<Shape>> = shapes.iter().map(|s| all_orientations(s)).collect();

    // Create list of pieces to place (each piece is a shape index)
    let mut pieces: Vec<usize> = Vec::new();
    for (idx, &qty) in quantities.iter().enumerate() {
        for _ in 0..qty {
            pieces.push(idx);
        }
    }

    // Sort pieces by number of orientations (less flexible first) and size (larger first)
    pieces.sort_by_key(|&idx| {
        (all_orientations_vec[idx].len(), -(shapes[idx].len() as i32))
    });

    let mut grid = vec![vec![false; width]; height];

    solve(&mut grid, &pieces, 0, &all_orientations_vec, width, height)
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let mut shapes: Vec<Shape> = Vec::new();
    let mut regions: Vec<(usize, usize, Vec<usize>)> = Vec::new();

    let mut i = 0;

    // Parse shapes
    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        // Check if it's a shape definition (digit followed by colon)
        if line.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) && line.contains(':') {
            // Check if it's a region definition (contains 'x')
            if line.contains('x') {
                break;
            }

            // It's a shape definition
            i += 1;
            let mut shape_lines: Vec<&str> = Vec::new();
            while i < lines.len() && !lines[i].trim().is_empty() &&
                  !lines[i].trim().chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                shape_lines.push(lines[i].trim());
                i += 1;
            }
            shapes.push(parse_shape(&shape_lines));
        } else {
            i += 1;
        }
    }

    // Parse regions
    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        // Parse region: WxH: q0 q1 q2 ...
        if line.contains('x') && line.contains(':') {
            let parts: Vec<&str> = line.split(':').collect();
            let dims: Vec<usize> = parts[0].split('x').map(|s| s.trim().parse().unwrap()).collect();
            let width = dims[0];
            let height = dims[1];
            let quantities: Vec<usize> = parts[1].trim().split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            regions.push((width, height, quantities));
        }

        i += 1;
    }

    let mut count = 0;
    for (width, height, quantities) in &regions {
        if can_fit_all(&shapes, *width, *height, quantities) {
            count += 1;
        }
    }

    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shape() {
        let lines = vec!["###", "##.", "##."];
        let shape = parse_shape(&lines);
        // Shape 0: ###
        //          ##.
        //          ##.
        // Cells: (0,0), (1,0), (2,0), (0,1), (1,1), (0,2), (1,2) = 7 cells
        assert_eq!(shape.len(), 7);
    }

    #[test]
    fn test_shape_4() {
        // Shape 4:
        // ###
        // #..
        // ###
        let lines = vec!["###", "#..", "###"];
        let shape = parse_shape(&lines);
        // Cells: (0,0), (1,0), (2,0), (0,1), (0,2), (1,2), (2,2) = 7 cells
        assert_eq!(shape.len(), 7);
    }

    #[test]
    fn test_4x4_two_shape4() {
        // 4x4 grid with 2 of shape 4 (###, #.., ###)
        let shape4 = parse_shape(&vec!["###", "#..", "###"]);
        let shapes = vec![shape4];
        let quantities = vec![2];

        let result = can_fit_all(&shapes, 4, 4, &quantities);
        assert!(result, "Should be able to fit 2 shape-4 pieces in 4x4");
    }

    #[test]
    fn test_example() {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

        let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

        let mut shapes: Vec<Shape> = Vec::new();
        let mut regions: Vec<(usize, usize, Vec<usize>)> = Vec::new();

        let mut i = 0;

        // Parse shapes
        while i < lines.len() {
            let line = lines[i].trim();

            if line.is_empty() {
                i += 1;
                continue;
            }

            if line.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) && line.contains(':') {
                if line.contains('x') {
                    break;
                }

                i += 1;
                let mut shape_lines: Vec<&str> = Vec::new();
                while i < lines.len() && !lines[i].trim().is_empty() &&
                      !lines[i].trim().chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                    shape_lines.push(lines[i].trim());
                    i += 1;
                }
                shapes.push(parse_shape(&shape_lines));
            } else {
                i += 1;
            }
        }

        while i < lines.len() {
            let line = lines[i].trim();

            if line.is_empty() {
                i += 1;
                continue;
            }

            if line.contains('x') && line.contains(':') {
                let parts: Vec<&str> = line.split(':').collect();
                let dims: Vec<usize> = parts[0].split('x').map(|s| s.trim().parse().unwrap()).collect();
                let width = dims[0];
                let height = dims[1];
                let quantities: Vec<usize> = parts[1].trim().split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                regions.push((width, height, quantities));
            }

            i += 1;
        }

        eprintln!("Shapes: {:?}", shapes);
        eprintln!("Regions: {:?}", regions);

        let mut count = 0;
        for (idx, (width, height, quantities)) in regions.iter().enumerate() {
            eprintln!("Testing region {}: {}x{} with {:?}", idx, width, height, quantities);
            if can_fit_all(&shapes, *width, *height, quantities) {
                eprintln!("  -> CAN FIT");
                count += 1;
            } else {
                eprintln!("  -> CANNOT FIT");
            }
        }

        assert_eq!(count, 2);
    }
}
