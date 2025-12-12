use std::io::{self, BufRead};
use std::collections::HashSet;

fn solve_puzzle(input: &str) -> i64 {
    // Parse red tile coordinates
    let red_tiles: Vec<(i64, i64)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<i64> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    if red_tiles.len() < 2 {
        return 0;
    }

    let n = red_tiles.len();

    // Build the polygon edges (as segments)
    // Each edge is (x1, y1, x2, y2) where either x1==x2 (vertical) or y1==y2 (horizontal)
    let mut edges: Vec<(i64, i64, i64, i64)> = Vec::new();
    for i in 0..n {
        let (x1, y1) = red_tiles[i];
        let (x2, y2) = red_tiles[(i + 1) % n];
        edges.push((x1, y1, x2, y2));
    }

    // Build set of red tiles for quick lookup
    let red_set: HashSet<(i64, i64)> = red_tiles.iter().cloned().collect();

    let mut max_area: i64 = 0;

    // Try all pairs of red tiles as opposite corners
    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];

            // Skip if same row or column (degenerate rectangle)
            if x1 == x2 || y1 == y2 {
                continue;
            }

            // Rectangle bounds
            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);

            let width = max_x - min_x + 1;
            let height = max_y - min_y + 1;
            let area = width * height;

            if area <= max_area {
                continue;
            }

            // Check if all 4 edges of the rectangle are inside or on the polygon
            // For a rectilinear polygon, we need to verify each edge of the rectangle
            // is either on a polygon edge or entirely inside the polygon

            if rectangle_inside_polygon(&edges, &red_tiles, min_x, max_x, min_y, max_y) {
                max_area = area;
            }
        }
    }

    max_area
}

// Check if rectangle is entirely inside or on the polygon boundary
fn rectangle_inside_polygon(
    edges: &[(i64, i64, i64, i64)],
    polygon: &[(i64, i64)],
    min_x: i64, max_x: i64, min_y: i64, max_y: i64
) -> bool {
    // For a rectilinear polygon, check if all 4 corners and sample points on edges are inside

    // Check 4 corners
    let corners = [
        (min_x, min_y),
        (min_x, max_y),
        (max_x, min_y),
        (max_x, max_y),
    ];

    for &(x, y) in &corners {
        if !is_inside_or_boundary_fast(edges, polygon, x, y) {
            return false;
        }
    }

    // Check midpoints of the 4 edges (to catch cases where corners are inside but edges cross outside)
    let mid_x = (min_x + max_x) / 2;
    let mid_y = (min_y + max_y) / 2;

    let edge_midpoints = [
        (min_x, mid_y),  // left edge
        (max_x, mid_y),  // right edge
        (mid_x, min_y),  // bottom edge
        (mid_x, max_y),  // top edge
    ];

    for &(x, y) in &edge_midpoints {
        if !is_inside_or_boundary_fast(edges, polygon, x, y) {
            return false;
        }
    }

    // Check center of rectangle
    if !is_inside_or_boundary_fast(edges, polygon, mid_x, mid_y) {
        return false;
    }

    // For rectilinear polygons, we need more thorough edge checking
    // Check if any polygon edge crosses our rectangle edges in a way that would exclude parts

    // Actually, for complete correctness with rectilinear polygons:
    // We need to check that the rectangle edges don't cross outside the polygon
    // This happens when a polygon edge "cuts into" our rectangle

    // Check each polygon edge for intersection with our rectangle
    for &(ex1, ey1, ex2, ey2) in edges {
        // Normalize edge coordinates
        let e_min_x = ex1.min(ex2);
        let e_max_x = ex1.max(ex2);
        let e_min_y = ey1.min(ey2);
        let e_max_y = ey1.max(ey2);

        if ex1 == ex2 {
            // Vertical polygon edge
            let ex = ex1;
            // Does it cross through our rectangle's interior?
            if ex > min_x && ex < max_x {
                // Edge is inside our x-range
                // Check if it overlaps our y-range
                if e_max_y > min_y && e_min_y < max_y {
                    // The polygon edge crosses through our rectangle
                    // This creates a potential cut - need to check more carefully
                    // Sample points along the cut
                    let check_y = (e_min_y.max(min_y) + e_max_y.min(max_y)) / 2;
                    // Check points on both sides of this vertical edge within our rectangle
                    if ex > min_x && !is_inside_or_boundary_fast(edges, polygon, ex - 1, check_y) {
                        return false;
                    }
                    if ex < max_x && !is_inside_or_boundary_fast(edges, polygon, ex + 1, check_y) {
                        return false;
                    }
                }
            }
        } else if ey1 == ey2 {
            // Horizontal polygon edge
            let ey = ey1;
            // Does it cross through our rectangle's interior?
            if ey > min_y && ey < max_y {
                // Edge is inside our y-range
                // Check if it overlaps our x-range
                if e_max_x > min_x && e_min_x < max_x {
                    // The polygon edge crosses through our rectangle
                    let check_x = (e_min_x.max(min_x) + e_max_x.min(max_x)) / 2;
                    // Check points on both sides of this horizontal edge within our rectangle
                    if ey > min_y && !is_inside_or_boundary_fast(edges, polygon, check_x, ey - 1) {
                        return false;
                    }
                    if ey < max_y && !is_inside_or_boundary_fast(edges, polygon, check_x, ey + 1) {
                        return false;
                    }
                }
            }
        }
    }

    true
}

// Check if point is on polygon boundary
fn is_on_boundary(edges: &[(i64, i64, i64, i64)], x: i64, y: i64) -> bool {
    for &(ex1, ey1, ex2, ey2) in edges {
        if ex1 == ex2 {
            // Vertical edge
            let min_y = ey1.min(ey2);
            let max_y = ey1.max(ey2);
            if x == ex1 && y >= min_y && y <= max_y {
                return true;
            }
        } else if ey1 == ey2 {
            // Horizontal edge
            let min_x = ex1.min(ex2);
            let max_x = ex1.max(ex2);
            if y == ey1 && x >= min_x && x <= max_x {
                return true;
            }
        }
    }
    false
}

// Check if point is inside polygon using ray casting
fn is_inside_polygon(polygon: &[(i64, i64)], x: i64, y: i64) -> bool {
    let n = polygon.len();
    let mut inside = false;

    let px = x as f64;
    let py = y as f64;

    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];

        let vx1 = x1 as f64;
        let vy1 = y1 as f64;
        let vx2 = x2 as f64;
        let vy2 = y2 as f64;

        if (vy1 > py) != (vy2 > py) {
            let x_intersect = vx1 + (py - vy1) / (vy2 - vy1) * (vx2 - vx1);
            if px < x_intersect {
                inside = !inside;
            }
        }
    }

    inside
}

fn is_inside_or_boundary_fast(edges: &[(i64, i64, i64, i64)], polygon: &[(i64, i64)], x: i64, y: i64) -> bool {
    is_on_boundary(edges, x, y) || is_inside_polygon(polygon, x, y)
}

fn main() {
    let stdin = io::stdin();
    let input: String = stdin.lock().lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    let result = solve_puzzle(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level2_example() {
        let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";
        assert_eq!(solve_puzzle(input), 24);
    }
}
