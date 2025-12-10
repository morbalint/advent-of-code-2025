use std::fs;
use num_bigint::BigInt;
use num_traits::Signed;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("data/input.txt")
        .expect("Failed to read input file");

    let coordinates: Vec<(BigInt, BigInt)> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            let x = BigInt::from_str(parts[0].trim()).expect("Invalid x coordinate");
            let y = BigInt::from_str(parts[1].trim()).expect("Invalid y coordinate");
            (x, y)
        })
        .collect();

    println!("Read {} coordinates:", coordinates.len());

    let mut max_area = BigInt::from(0);
    let mut max_pair: Option<(usize, usize)> = None;

    // Pair each coordinate with every other coordinate
    for i in 0..coordinates.len() {
        for j in (i + 1)..coordinates.len() {
            let (x1, y1) = &coordinates[i];
            let (x2, y2) = &coordinates[j];

            // Calculate the area of the rectangle
            // Coordinates are inclusive, so we add 1 to both dimensions
            // For example: (4,5) to (4,7) = (4-4+1) * (7-5+1) = 1 * 3 = 3
            let width = (x2 - x1).abs() + 1;
            let height = (y2 - y1).abs() + 1;
            let area = &width * &height;

            if area > max_area {
                max_area = area;
                max_pair = Some((i, j));
            }
        }
    }

    if let Some((i, j)) = max_pair {
        let (x1, y1) = &coordinates[i];
        let (x2, y2) = &coordinates[j];
        println!("\nLargest rectangle:");
        println!("  Corners: ({}, {}) and ({}, {})", x1, y1, x2, y2);
        println!("  Width: {}", (x2 - x1).abs() + 1);
        println!("  Height: {}", (y2 - y1).abs() + 1);
        println!("  Area: {}", max_area);
    } else {
        println!("No coordinates found");
    }
}
