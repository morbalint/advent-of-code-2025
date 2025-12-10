use std::fs;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive};
use std::str::FromStr;

// Check if a point is inside a polygon using ray casting algorithm
fn point_in_polygon(px: i64, py: i64, polygon: &[(i64, i64)]) -> bool {
    let mut inside = false;
    let n = polygon.len();

    // First check if the point is a vertex of the polygon
    for &(vx, vy) in polygon {
        if px == vx && py == vy {
            return true;
        }
    }

    // Check if point is on an edge
    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];

        // Check if point is on this edge (between the two vertices)
        if point_on_segment(px, py, x1, y1, x2, y2) {
            return true;
        }
    }

    // Ray casting for interior points
    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];

        if ((y1 > py) != (y2 > py)) && (px < (x2 - x1) * (py - y1) / (y2 - y1) + x1) {
            inside = !inside;
        }
    }

    inside
}

// Check if point (px, py) is on the line segment from (x1, y1) to (x2, y2)
fn point_on_segment(px: i64, py: i64, x1: i64, y1: i64, x2: i64, y2: i64) -> bool {
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);

    // Point must be within bounding box
    if px < min_x || px > max_x || py < min_y || py > max_y {
        return false;
    }

    // Check collinearity using cross product
    // (px - x1, py - y1) × (x2 - x1, y2 - y1) = 0
    let cross = (px - x1) * (y2 - y1) - (py - y1) * (x2 - x1);
    cross == 0
}

// Check if a horizontal line segment intersects with a vertical line segment
fn h_v_segments_intersect(hx1: i64, hx2: i64, hy: i64, vx: i64, vy1: i64, vy2: i64) -> bool {
    let h_min_x = hx1.min(hx2);
    let h_max_x = hx1.max(hx2);
    let v_min_y = vy1.min(vy2);
    let v_max_y = vy1.max(vy2);

    // Check if they intersect
    vx >= h_min_x && vx <= h_max_x && hy >= v_min_y && hy <= v_max_y
}

// Check if a vertical line segment intersects with a horizontal line segment
fn v_h_segments_intersect(vx: i64, vy1: i64, vy2: i64, hx1: i64, hx2: i64, hy: i64) -> bool {
    h_v_segments_intersect(hx1, hx2, hy, vx, vy1, vy2)
}

// Check if two horizontal segments overlap (not just touch at endpoints)
fn h_h_segments_overlap(x1a: i64, x1b: i64, y1: i64, x2a: i64, x2b: i64, y2: i64) -> bool {
    if y1 != y2 {
        return false;
    }
    let min1 = x1a.min(x1b);
    let max1 = x1a.max(x1b);
    let min2 = x2a.min(x2b);
    let max2 = x2a.max(x2b);

    // Check for overlap (not just touching at endpoints)
    max1 > min2 && max2 > min1
}

// Check if two vertical segments overlap (not just touch at endpoints)
fn v_v_segments_overlap(x1: i64, y1a: i64, y1b: i64, x2: i64, y2a: i64, y2b: i64) -> bool {
    if x1 != x2 {
        return false;
    }
    let min1 = y1a.min(y1b);
    let max1 = y1a.max(y1b);
    let min2 = y2a.min(y2b);
    let max2 = y2a.max(y2b);

    // Check for overlap (not just touching at endpoints)
    max1 > min2 && max2 > min1
}

// Check if all four corners of a rectangle are inside or on the polygon boundary
fn rectangle_in_polygon(x1: i64, y1: i64, x2: i64, y2: i64, polygon: &[(i64, i64)]) -> bool {
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);

    // Check all four corners are inside the polygon
    let corners = [
        (min_x, min_y),
        (min_x, max_y),
        (max_x, min_y),
        (max_x, max_y),
    ];

    for &(cx, cy) in &corners {
        if !point_in_polygon(cx, cy, polygon) {
            return false;
        }
    }

    // For a concave polygon, we need to check that no polygon edge crosses through
    // the interior of the rectangle. A polygon edge crosses through if it has
    // one endpoint strictly inside (not on boundary) and one outside the rectangle.

    let n = polygon.len();
    for i in 0..n {
        let (px1, py1) = polygon[i];
        let (px2, py2) = polygon[(i + 1) % n];

        // Check if endpoints are inside/on the rectangle
        let p1_in_or_on_rect = px1 >= min_x && px1 <= max_x && py1 >= min_y && py1 <= max_y;
        let p2_in_or_on_rect = px2 >= min_x && px2 <= max_x && py2 >= min_y && py2 <= max_y;

        // Check if endpoints are strictly inside (not on boundary)
        let p1_strictly_inside = px1 > min_x && px1 < max_x && py1 > min_y && py1 < max_y;
        let p2_strictly_inside = px2 > min_x && px2 < max_x && py2 > min_y && py2 < max_y;

        // If one endpoint is strictly inside and one is outside, the edge crosses
        if (p1_strictly_inside && !p2_in_or_on_rect) || (p2_strictly_inside && !p1_in_or_on_rect) {
            return false;
        }
    }

    // For small to medium rectangles, check all interior points to be absolutely sure
    // This catches edge cases with concave polygons
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    if width * height <= 10000 {
        // Check every point
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if !point_in_polygon(x, y, polygon) {
                    return false;
                }
            }
        }
    }


    true
}

// Visualize the polygon on a 2D grid
fn visualize_polygon(polygon: &[(i64, i64)]) {
    use std::io::Write;

    if polygon.is_empty() {
        return;
    }

    // Find bounding box
    let min_x = polygon.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = polygon.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = polygon.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = polygon.iter().map(|(_, y)| *y).max().unwrap();

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    println!("\nPolygon size: {}x{}", width, height);
    println!("Bounding box: ({},{}) to ({},{})", min_x, min_y, max_x, max_y);

    // Maximum reasonable size for visualization
    let max_dimension = 10000;
    if width > max_dimension || height > max_dimension {
        println!("Polygon too large to visualize (would take too long)");
        println!("To visualize anyway, you would need to process {} points", (width as u64) * (height as u64));
        return;
    }

    // Decide whether to print to console or file
    let to_file = width > 200 || height > 200;

    if to_file {
        println!("Polygon too large for console - saving to map.txt...");
    }

    // Create grid
    let mut grid = vec![vec![' '; width]; height];

    // Fill polygon interior and edges
    println!("Generating visualization...");
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if point_in_polygon(x, y, polygon) {
                let grid_y = (y - min_y) as usize;
                let grid_x = (x - min_x) as usize;
                grid[grid_y][grid_x] = '#';
            }
        }
    }

    // Mark vertices with 'O'
    for &(vx, vy) in polygon {
        let grid_y = (vy - min_y) as usize;
        let grid_x = (vx - min_x) as usize;
        grid[grid_y][grid_x] = 'O';
    }

    if to_file {
        // Write to file
        let mut file = fs::File::create("map.txt").expect("Failed to create map.txt");
        writeln!(file, "Polygon visualization").unwrap();
        writeln!(file, "X: {} to {}, Y: {} to {}", min_x, max_x, min_y, max_y).unwrap();
        writeln!(file, "Size: {}x{}", width, height).unwrap();
        writeln!(file).unwrap();

        for row in &grid {
            for &cell in row {
                write!(file, "{}", cell).unwrap();
            }
            writeln!(file).unwrap();
        }

        println!("Visualization saved to map.txt");
    } else {
        // Print to console
        println!("\nPolygon visualization:");
        println!("  X: {} to {}, Y: {} to {}", min_x, max_x, min_y, max_y);
        println!();

        for row in &grid {
            print!("  ");
            for &cell in row {
                print!("{}", cell);
            }
            println!();
        }
        println!();
    }
}

fn main() {
    // Try input.txt first, fall back to test inputs
    let input = fs::read_to_string("data/input.txt")
        .or_else(|_| fs::read_to_string("data/test-input.txt"))
        .or_else(|_| fs::read_to_string("data/test-input-2.txt"))
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

    println!("Read {} coordinates forming a polygon", coordinates.len());

    // Convert to i64 for geometric calculations
    let polygon: Vec<(i64, i64)> = coordinates
        .iter()
        .map(|(x, y)| {
            (x.to_i64().expect("Coordinate too large for i64"),
             y.to_i64().expect("Coordinate too large for i64"))
        })
        .collect();

    // Visualize the polygon
    visualize_polygon(&polygon);

    let mut max_area = BigInt::from(0);
    let mut max_pair: Option<(usize, usize)> = None;

    println!("\nFinding valid rectangles...");

    // Pair each coordinate with every other coordinate
    for i in 0..coordinates.len() {
        for j in (i + 1)..coordinates.len() {
            let (x1, y1) = polygon[i];
            let (x2, y2) = polygon[j];

            // Check if the rectangle with these diagonal corners is inside the polygon
            if rectangle_in_polygon(x1, y1, x2, y2, &polygon) {
                // Calculate the area of the rectangle
                // Coordinates are inclusive, so we add 1 to both dimensions
                let width = BigInt::from((x2 - x1).abs()) + 1;
                let height = BigInt::from((y2 - y1).abs()) + 1;
                let area = &width * &height;

                if area > max_area {
                    max_area = area;
                    max_pair = Some((i, j));
                }
            }
        }
    }

    if let Some((i, j)) = max_pair {
        let (x1, y1) = &coordinates[i];
        let (x2, y2) = &coordinates[j];
        println!("\nLargest rectangle contained in polygon:");
        println!("  Corners: ({}, {}) and ({}, {})", x1, y1, x2, y2);
        println!("  Width: {}", (x2 - x1).abs() + 1);
        println!("  Height: {}", (y2 - y1).abs() + 1);
        println!("  Area: {}", max_area);

        // Visualize the rectangle on the polygon
        let rect_x1 = x1.to_i64().unwrap();
        let rect_y1 = y1.to_i64().unwrap();
        let rect_x2 = x2.to_i64().unwrap();
        let rect_y2 = y2.to_i64().unwrap();
        visualize_rectangle_on_polygon(&polygon, rect_x1, rect_y1, rect_x2, rect_y2);
    } else {
        println!("No valid rectangle found inside the polygon");
    }
}

fn visualize_rectangle_on_polygon(polygon: &[(i64, i64)], rx1: i64, ry1: i64, rx2: i64, ry2: i64) {
    if polygon.is_empty() {
        return;
    }

    let min_x = polygon.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = polygon.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = polygon.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = polygon.iter().map(|(_, y)| *y).max().unwrap();

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    if width > 100 || height > 100 {
        return; // Skip if too large
    }

    let rect_min_x = rx1.min(rx2);
    let rect_max_x = rx1.max(rx2);
    let rect_min_y = ry1.min(ry2);
    let rect_max_y = ry1.max(ry2);

    println!("\nRectangle overlay (R = rectangle only, X = rectangle outside polygon):");
    let mut grid = vec![vec![' '; width]; height];

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let grid_y = (y - min_y) as usize;
            let grid_x = (x - min_x) as usize;

            let in_poly = point_in_polygon(x, y, polygon);
            let in_rect = x >= rect_min_x && x <= rect_max_x && y >= rect_min_y && y <= rect_max_y;

            grid[grid_y][grid_x] = match (in_poly, in_rect) {
                (true, true) => 'R',   // Inside both
                (true, false) => '#',  // Polygon only
                (false, true) => 'X',  // Rectangle extends outside!
                (false, false) => ' ', // Outside both
            };
        }
    }

    // Mark rectangle corners
    if rect_min_y >= min_y && rect_min_y <= max_y && rect_min_x >= min_x && rect_min_x <= max_x {
        grid[(rect_min_y - min_y) as usize][(rect_min_x - min_x) as usize] = 'C';
    }
    if rect_max_y >= min_y && rect_max_y <= max_y && rect_max_x >= min_x && rect_max_x <= max_x {
        grid[(rect_max_y - min_y) as usize][(rect_max_x - min_x) as usize] = 'C';
    }
    if rect_min_y >= min_y && rect_min_y <= max_y && rect_max_x >= min_x && rect_max_x <= max_x {
        grid[(rect_min_y - min_y) as usize][(rect_max_x - min_x) as usize] = 'C';
    }
    if rect_max_y >= min_y && rect_max_y <= max_y && rect_min_x >= min_x && rect_min_x <= max_x {
        grid[(rect_max_y - min_y) as usize][(rect_min_x - min_x) as usize] = 'C';
    }

    for row in &grid {
        print!("  ");
        for &cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!("  Legend: C=corners, R=valid rect area, #=polygon only, X=ERROR rect outside polygon");
}

