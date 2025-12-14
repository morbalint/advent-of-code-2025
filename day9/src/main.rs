use std::fs;
use num_traits::ToPrimitive;
use std::str::FromStr;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Rectangle {
    index_i: usize,
    index_j: usize,
    min_x: u64,
    max_x: u64,
    min_y: u64,
    max_y: u64,
    area: u64,
}

pub fn check_rectangle_validity(
    rect: &Rectangle,
    coordinates: &[(u64, u64)],
) -> bool {

    let n = coordinates.len();

    // SETUP HERE
    // Calculate the height and width for the boolean arrays
    let height: usize = (&rect.max_y - &rect.min_y).to_usize().unwrap() + 1;
    let width: usize = (&rect.max_x - &rect.min_x).to_usize().unwrap() + 1;

    // Boolean array for left edge (x = min_x, y from min_y to max_y, corners included)
    let mut left_edge = vec![false; height];

    // Boolean array for top edge (y = min_y, x from min_x to max_x, corners included)
    let mut top_edge = vec![false; width];

    for k in 0..n {
        let (x_k, y_k) = &coordinates[k];
        // check for inner point
        if (x_k > &rect.min_x && x_k < &rect.max_x) && (y_k > &rect.min_y && y_k < &rect.max_y) {
            return false;
        }

        let k_plus_1 = (k + 1) % n;
        let (x_kp1, y_kp1) = &coordinates[k_plus_1];
        let k_plus_2 = (k + 2) % n;
        let (x_kp2, y_kp2) = &coordinates[k_plus_2];
        
        let k_minus_1 = (k + n - 1) % n;
        let (x_km1, y_km1) = &coordinates[k_minus_1];
        

        let is_vertical_edge = x_k == x_kp1;
        let is_horizontal_edge = y_k == y_kp1;

        if is_vertical_edge {

            if x_k > &rect.max_x {
                continue;
            }

            let smaller_y = y_k.min(y_kp1);
            let larger_y = y_k.max(y_kp1);

            // On the top side with at least two points
            if x_k >= &rect.min_x && larger_y <= &rect.min_y {
                // don't flip the bit for U-turn
                let is_uturn = (x_kp2 > x_k && x_km1 > x_k) || (x_kp2 < x_k && x_km1 < x_k);
                if !is_uturn {
                    let idx_to_flip = (x_k - &rect.min_x).to_usize().unwrap();
                    top_edge[idx_to_flip] = !top_edge[idx_to_flip];    
                }
            }

            if x_k > &rect.min_x {
                continue;
            }

            if smaller_y > &rect.max_y {
                continue;
            }
            if larger_y < &rect.min_y {
                continue;
            }

            let start_y = (smaller_y.max(&rect.min_y) - &rect.min_y).to_usize().unwrap();
            let end_y = (larger_y.min(&rect.max_y) - &rect.min_y).to_usize().unwrap() + 1; // +1 to include the endpoint
            for y in start_y..end_y {
                left_edge[y] = !left_edge[y];
            }
        }

        if is_horizontal_edge {

            if y_k > &rect.max_y {
                continue;
            }

            let smaller_x = x_k.min(x_kp1);
            let larger_x = x_k.max(x_kp1);

            // On the left side with at least two point
            if y_k >= &rect.min_y && larger_x <= &rect.min_x {
                let is_uturn = (y_kp2 > y_k && y_km1 > y_k) || (y_kp2 < y_k && y_km1 < y_k);
                if !is_uturn {
                    let idx_to_flip = (y_k - &rect.min_y).to_usize().unwrap();
                    left_edge[idx_to_flip] = !left_edge[idx_to_flip];    
                }
            }

            if y_k > &rect.min_y {
                continue;
            }

            if smaller_x > &rect.max_x {
                continue;
            }
            if larger_x < &rect.min_x {
                continue;
            }

            let start_x = (smaller_x.max(&rect.min_x) - &rect.min_x).to_usize().unwrap();
            let end_x = (larger_x.min(&rect.max_x) - &rect.min_x).to_usize().unwrap() + 1; // +1 to include the endpoint
            for x in start_x..end_x {
                top_edge[x] = !top_edge[x];
            }
        }
    }

    if left_edge.iter().any(|&v| !v) {
        return false;
    }
    if top_edge.iter().any(|&v| !v) {
        return false;
    }

    true
}

fn main() {
    let input = fs::read_to_string("data/input.txt")
        .expect("Failed to read input file");

    let coordinates: Vec<(u64, u64)> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            let x = u64::from_str(parts[0].trim()).expect("Invalid x coordinate");
            let y = u64::from_str(parts[1].trim()).expect("Invalid y coordinate");
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

pub fn find_largest_valid_rectangle(input: &str) -> Option<u64> {
    let coordinates: Vec<(u64, u64)> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            let x = u64::from_str(parts[0].trim()).expect("Invalid x coordinate");
            let y = u64::from_str(parts[1].trim()).expect("Invalid y coordinate");
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


