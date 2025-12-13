use std::fs;
use num_bigint::BigInt;
use std::str::FromStr;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Rectangle {
    index_i: usize,
    index_j: usize,
    min_x: BigInt,
    max_x: BigInt,
    min_y: BigInt,
    max_y: BigInt,
    area: BigInt,
}

pub fn check_rectangle_validity(
    rect: &Rectangle,
    coordinates: &[(BigInt, BigInt)],
) -> bool {
    let i = rect.index_i;
    let j = rect.index_j;

    let (x_i, y_i) = &coordinates[i];
    let (x_j, y_j) = &coordinates[j];

    let n = coordinates.len();

    // For inner chain: from i to j
    let mut inner_chain_reached_x = false;
    let mut inner_chain_reached_y = false;

    // Determine what we're looking for based on starting and ending coordinates
    let inner_looking_for_x_gte = x_i < x_j;
    let inner_looking_for_y_gte = y_i < y_j;

    let mut inner_chain_x_before_y : Option<bool> = None;

    // For inner chain: from i to j
    let mut outer_chain_reached_x = false;
    let mut outer_chain_reached_y = false;

    // Determine what we're looking for based on starting and ending coordinates
    let outer_looking_for_x_gte = !inner_looking_for_x_gte;
    let outer_looking_for_y_gte = !inner_looking_for_y_gte;

    let mut outer_chain_x_before_y : Option<bool> = None;

    for k in 0..n {
        let (x_k, y_k) = &coordinates[k];
        // check for inner point
        if (x_k > &rect.min_x && x_k < &rect.max_x) && (y_k > &rect.min_y && y_k < &rect.max_y) {
            return false;
        }

        let is_inner_chain = (k > i) && (k <= j);
        if is_inner_chain {
            // Check for reaching x boundary
            if inner_looking_for_x_gte {
                if x_k >= &rect.max_x {
                    inner_chain_reached_x = true;
                    if inner_chain_reached_y {
                        inner_chain_x_before_y = Some(false);
                    }
                }
                else {
                    if x_k < &rect.max_x {
                        inner_chain_reached_x = false;
                    }
                }
            } else {
                if x_k <= &rect.min_x {
                    inner_chain_reached_x = true;
                    if inner_chain_reached_y {
                        inner_chain_x_before_y = Some(false);
                    }
                }
                else {
                    if x_k > &rect.min_x {
                        inner_chain_reached_x = false;
                    }
                }
            }

            // Check for reaching y boundary
            if inner_looking_for_y_gte {
                if y_k >= &rect.max_y {
                    inner_chain_reached_y = true;
                    if inner_chain_reached_x {
                        inner_chain_x_before_y = Some(true);
                    }
                }
                else {
                    if y_k < &rect.max_y {
                        inner_chain_reached_y = false;
                    }
                }
            } else {
                if y_k <= &rect.min_y {
                    inner_chain_reached_y = true;
                    if inner_chain_reached_x {
                        inner_chain_x_before_y = Some(true);
                    }
                }
                else {
                    if y_k > &rect.min_y {
                        inner_chain_reached_y = false;
                    }
                }
            }
        } else {
            // Outer chain
            // Check for reaching x boundary
            if outer_looking_for_x_gte {
                if x_k >= &rect.max_x {
                    outer_chain_reached_x = true;
                    if outer_chain_reached_y {
                        outer_chain_x_before_y = Some(false);
                    }
                }
                else {
                    if x_k < &rect.max_x {
                        outer_chain_reached_x = false;
                    }
                }
            } else {
                if x_k <= &rect.min_x {
                    outer_chain_reached_x = true;
                    if outer_chain_reached_y {
                        outer_chain_x_before_y = Some(false);
                    }
                }
                else {
                    if x_k > &rect.min_x {
                        outer_chain_reached_x = false;
                    }
                }
            }

            // Check for reaching y boundary
            if outer_looking_for_y_gte {
                if y_k >= &rect.max_y {
                    outer_chain_reached_y = true;
                    if outer_chain_reached_x {
                        outer_chain_x_before_y = Some(true);
                    }
                }
                else {
                    if y_k < &rect.max_y {
                        outer_chain_reached_y = false;
                    }
                }
            } else {
                if y_k <= &rect.min_y {
                    outer_chain_reached_y = true;
                    if outer_chain_reached_x {
                        outer_chain_x_before_y = Some(true);
                    }
                }
                else {
                    if y_k > &rect.min_y {
                        outer_chain_reached_y = false;
                    }
                }
            }
        }
    }

    if inner_chain_x_before_y.unwrap() ^ outer_chain_x_before_y.unwrap() {
        return true;
    }

    false
}

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

    // Step 1: Generate all possible rectangles
    let mut rectangles = Vec::new();

    for i in 0..coordinates.len() {
        for j in (i + 1)..coordinates.len() {
            let (x1, y1) = &coordinates[i];
            let (x2, y2) = &coordinates[j];

            // Get the min and max coordinates to define the rectangle bounds
            let min_x = x1.min(x2).clone();
            let max_x = x1.max(x2).clone();
            let min_y = y1.min(y2).clone();
            let max_y = y1.max(y2).clone();

            // Calculate the area of the rectangle
            // Coordinates are inclusive, so we add 1 to both dimensions
            let width = (&max_x - &min_x) + 1;
            let height = (&max_y - &min_y) + 1;
            let area = &width * &height;

            rectangles.push(Rectangle {
                index_i: i,
                index_j: j,
                min_x,
                max_x,
                min_y,
                max_y,
                area,
            });
        }
    }

    println!("Generated {} rectangles", rectangles.len());

    // Step 2: Sort rectangles by area (largest first)
    rectangles.sort_by(|a, b| b.area.cmp(&a.area));

    // Step 3: Find the first (largest) rectangle that satisfies all conditions
    let mut found_rectangle: Option<&Rectangle> = None;

    for rect in &rectangles {
        if check_rectangle_validity(rect, &coordinates) {
            found_rectangle = Some(rect);
            break;
        }
    }

    // Step 4: Display results
    if let Some(rect) = found_rectangle {
        let (x1, y1) = &coordinates[rect.index_i];
        let (x2, y2) = &coordinates[rect.index_j];
        println!("\nLargest valid rectangle:");
        println!("  Corners: ({}, {}) and ({}, {})", x1, y1, x2, y2);
        println!("  Width: {}", (&rect.max_x - &rect.min_x) + 1);
        println!("  Height: {}", (&rect.max_y - &rect.min_y) + 1);
        println!("  Area: {}", rect.area);
    } else {
        println!("No valid rectangle found");
    }
}

pub fn find_largest_valid_rectangle(input: &str) -> Option<BigInt> {
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

    // Generate all possible rectangles
    let mut rectangles = Vec::new();

    for i in 0..coordinates.len() {
        for j in (i + 1)..coordinates.len() {
            let (x1, y1) = &coordinates[i];
            let (x2, y2) = &coordinates[j];

            let min_x = x1.min(x2).clone();
            let max_x = x1.max(x2).clone();
            let min_y = y1.min(y2).clone();
            let max_y = y1.max(y2).clone();

            let width = (&max_x - &min_x) + 1;
            let height = (&max_y - &min_y) + 1;
            let area = &width * &height;

            rectangles.push(Rectangle {
                index_i: i,
                index_j: j,
                min_x,
                max_x,
                min_y,
                max_y,
                area,
            });
        }
    }

    // Sort rectangles by area (largest first)
    rectangles.sort_by(|a, b| b.area.cmp(&a.area));

    // Find the first (largest) rectangle that satisfies all conditions
    for rect in &rectangles {
        if check_rectangle_validity(rect, &coordinates) {
            return Some(rect.area.clone());
        }
    }

    None
}


