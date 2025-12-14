use super::*;

#[test]
fn test_example_from_file() {
    // Test input from test-input.txt
    let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    let result = find_largest_valid_rectangle(input);
    assert!(result.is_some(), "Should find a valid rectangle");
    // The actual largest valid rectangle based on the corner logic
    assert_eq!(result.unwrap(), 24_u64);
}

#[test]
fn test_simple_square() {
    // Four corners of a square with no interior points
    let input = "0,0
10,0
10,10
0,10";

    let result = find_largest_valid_rectangle(input);
    assert!(result.is_some(), "Should find a valid rectangle");
    // Rectangle from (0,0) to (10,10) = width: 11, height: 11 = area: 121
    assert_eq!(result.unwrap(), 121_u64);
}

#[test]
fn test_large_numbers() {
    // Test with larger coordinate values
    let input = "1000,2000
5000,2000
5000,8000
1000,8000
";

    let result = find_largest_valid_rectangle(input);
    assert!(result.is_some());
    // Rectangle from (1000,2000) to (5000,8000)
    // Width: 4001, Height: 6001, Area: 24,010,001
    assert_eq!(result.unwrap(), 24010001_u64);
}

#[test]
fn test_narrow_rectangle() {
    // Test a tall, narrow rectangle
    let input = "5,1
6,1
6,100
5,100";

    let result = find_largest_valid_rectangle(input);
    assert!(result.is_some());
    // Rectangle from (5,1) to (6,100)
    // Width: 2, Height: 100, Area: 200
    assert_eq!(result.unwrap(), 200_u64);
}

#[test]
fn test_single_point_rectangles() {
    // Test degenerate rectangles (width or height of 1)
    let input = "5,5
5,5
3,3
7,7";

    let result = find_largest_valid_rectangle(input);
    assert!(result.is_some());
    // Duplicate points and corner requirements result in smaller valid rectangles
    assert_eq!(result.unwrap(), 1_u64);
}

#[test]
fn test_l_shaped_boundary() {
    // L-shaped points creating multiple valid rectangles
    let input = "0,0
10,0
10,10
7,10
7,3
0,3";

    let result = find_largest_valid_rectangle(input);
    assert!(result.is_some());
    // Should find some valid rectangle
    let area = result.unwrap();
    assert_eq!(area, 44_u64);
}

#[test]
fn test_wierd_shaped_1_boundary() {
    // L-shaped points creating multiple valid rectangles
    let input = "5,5
5,0
10,0
10,4
36,4
36,30
40,30
40,35
35,35
35,40
30,40
30,36
4,36
4,10
0,10
0,5
";


    let result = find_largest_valid_rectangle(input);
    assert!(result.is_some());
    // Should find some valid rectangle
    let area = result.unwrap();
    assert_eq!(area, 961_u64);
}

#[test]
fn test_wierd_shaped_2_boundary() {
    // L-shaped points creating multiple valid rectangles
    let input = "10,10
10,5
5,5
5,0
15,0
15,5
115,5
115,115
0,115
0,105
5,105
5,110
110,110
110,10
";
    
    let result = find_largest_valid_rectangle(input);
    assert!(result.is_some());
    // Should find some valid rectangle
    let area = result.unwrap();
    assert_eq!(area, 636_u64);
}

#[test]
fn test_check_rectangle_validity_simple_valid() {
    // Simple square with all 4 corners
    let coordinates = vec![
        (0, 0),
        (10, 0),
        (10, 10),
        (0, 10),
    ];

    let rect = Rectangle {
        index_i: 0,
        index_j: 2,
        min_x: 0,
        max_x: 10,
        min_y: 0,
        max_y: 10,
        area: 121,
    };

    assert!(check_rectangle_validity(&rect, &coordinates));
}

#[test]
fn test_check_rectangle_validity_with_interior_point() {
    // Rectangle with an interior point should be invalid
    let coordinates = vec![
        (0, 0),
        (10, 0),
        (5, 5),  // Interior point
        (10, 10),
        (0, 10),
    ];

    let rect = Rectangle {
        index_i: 0,
        index_j: 3,
        min_x: 0,
        max_x: 10,
        min_y: 0,
        max_y: 10,
        area: 121,
    };

    assert!(!check_rectangle_validity(&rect, &coordinates));
}

#[test]
fn test_weird_shape2_winner() {
    let coordinates = vec![
        (10, 10),
        (10, 5),
        (5, 5),
        (5, 0),
        (15, 0),
        (15, 5),
        (115, 5),
        (115, 115),
        (0, 115),//-1
        (0, 105),
        (5, 105),
        (5, 110),
        (110, 110), //-2
        (110, 10),
    ];

    let rect = Rectangle {
        index_i: 8,
        index_j: 12,
        min_x: 0,
        max_x: 110,
        min_y: 110,
        max_y: 115,
        area: 666,
    };

    assert!(check_rectangle_validity(&rect, &coordinates));
}

#[test]
fn test_winner_from_test_input_txt() {
    let coordinates = vec![
        (7, 1),
        (11, 1),
        (11, 7),
        (9, 7),
        (9, 5),
        (2, 5),
        (2, 3),
        (7, 3),
    ];

    let rect = Rectangle {
        index_i: 4,
        index_j: 6,
        min_x: 2,
        max_x: 9,
        min_y: 3,
        max_y: 5,
        area: 24,
    };

    assert!(check_rectangle_validity(&rect, &coordinates));
}

// TODO: edge case: continue with point in middle of the edge
// #[test]
// fn test_check_rectangle_validity_with_complete_edges() {
//     // Rectangle with complete edges formed by multiple points
//     let coordinates = vec![
//         (0, 0),
//         (5, 0),
//         (10, 0),
//         (10, 5),
//         (10, 10),
//         (5, 10),
//         (0, 10),
//         (0, 5),
//     ];
//
//     let rect = Rectangle {
//         index_i: 0,
//         index_j: 4,
//         min_x: 0,
//         max_x: 10,
//         min_y: 0,
//         max_y: 10,
//         area: 121,
//     };
//
//     assert!(check_rectangle_validity(&rect, &coordinates));
// }

#[test]
fn test_check_rectangle_validity_point_on_boundary() {
    // Rectangle with a point on the boundary (should be valid)
    let coordinates = vec![
        (0, 0),
        (10, 0),
        (10, 5),  // Point on right edge
        (10, 10),
        (0, 10),
    ];

    let rect = Rectangle {
        index_i: 0,
        index_j: 3,
        min_x: 0,
        max_x: 10,
        min_y: 0,
        max_y: 10,
        area: 121,
    };

    assert!(check_rectangle_validity(&rect, &coordinates));
}

#[test]
fn test_check_rectangle_validity_degenerate_line() {
    // Degenerate rectangle (line - width of 1)
    let coordinates = vec![
        (5, 0),
        (5, 10),
    ];

    let rect = Rectangle {
        index_i: 0,
        index_j: 1,
        min_x: 5,
        max_x: 5,
        min_y: 0,
        max_y: 10,
        area: 11,
    };

    assert!(check_rectangle_validity(&rect, &coordinates));
}

#[test]
fn test_check_rectangle_validity_single_point() {
    // Degenerate rectangle (single point)
    let coordinates = vec![
        (5, 5),
    ];

    let rect = Rectangle {
        index_i: 0,
        index_j: 0,
        min_x: 5,
        max_x: 5,
        min_y: 5,
        max_y: 5,
        area: 1,
    };

    assert!(check_rectangle_validity(&rect, &coordinates));
}

#[test]
fn test_check_rectangle_validity_larger_boundary() {
    // Rectangle within a larger boundary shape
    let coordinates = vec![
        (0, 0),
        (20, 0),
        (20, 20),
        (0, 20),
    ];

    // Check a smaller rectangle within this boundary
    let rect = Rectangle {
        index_i: 0,
        index_j: 2,
        min_x: 5,
        max_x: 15,
        min_y: 5,
        max_y: 15,
        area: 121,
    };

    // Should be invalid because the edges don't cover this rectangle
    assert!(!check_rectangle_validity(&rect, &coordinates));
}

#[test]
fn test_check_rectangle_validity_offset_rectangle() {
    // Rectangle not starting at origin
    let coordinates = vec![
        (100, 200),
        (150, 200),
        (150, 250),
        (100, 250),
    ];

    let rect = Rectangle {
        index_i: 0,
        index_j: 2,
        min_x: 100,
        max_x: 150,
        min_y: 200,
        max_y: 250,
        area: 2601,
    };

    assert!(check_rectangle_validity(&rect, &coordinates));
}

#[test]
fn test_check_rectangle_validity_narrow_rectangle() {
    // Very narrow rectangle (2 units wide, 100 units tall)
    let coordinates = vec![
        (5, 0),
        (6, 0),
        (6, 100),
        (5, 100),
    ];

    let rect = Rectangle {
        index_i: 0,
        index_j: 2,
        min_x: 5,
        max_x: 6,
        min_y: 0,
        max_y: 100,
        area: 202,
    };

    assert!(check_rectangle_validity(&rect, &coordinates));
}
