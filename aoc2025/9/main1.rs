use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut points: Vec<(i64, i64)> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        let x: i64 = parts[0].parse().unwrap();
        let y: i64 = parts[1].parse().unwrap();
        points.push((x, y));
    }

    let mut max_area: i64 = 0;

    // Check all pairs of points
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            // Area of rectangle with opposite corners at (x1, y1) and (x2, y2)
            // The rectangle includes both endpoints, so we add 1 to each dimension
            let width = (x2 - x1).abs() + 1;
            let height = (y2 - y1).abs() + 1;
            let area = width * height;

            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("{}", max_area);
}
