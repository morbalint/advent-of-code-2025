use super::*;
use num_bigint::BigInt;

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
    assert_eq!(result.unwrap(), BigInt::from(24));
}

#[test]
fn test_simple_square() {
    // Four corners of a square with no interior points
    let input = "0,0
10,0
0,10
10,10";

    let result = find_largest_valid_rectangle(input);
    assert!(result.is_some(), "Should find a valid rectangle");
    // Rectangle from (0,0) to (10,10) = width: 11, height: 11 = area: 121
    assert_eq!(result.unwrap(), BigInt::from(121));
}

#[test]
fn test_with_interior_point() {
    // Rectangle with an interior point should be invalid
    let input = "0,0
10,0
5,5
0,10
10,10";

    let result = find_largest_valid_rectangle(input);
    // The largest valid rectangle should NOT be (0,0) to (10,10)
    // because (5,5) is strictly inside it
    // Should find smaller rectangles instead
    assert!(result.is_some());
    let area = result.unwrap();
    assert!(area < BigInt::from(121), "Should not include the rectangle with interior point");
}

#[test]
fn test_missing_corner() {
    // Rectangle missing one corner should be invalid
    let input = "0,0
10,0
0,10";
    // Missing corner at (10,10) or equivalent in other quadrants

    let result = find_largest_valid_rectangle(input);
    // May find smaller rectangles, but not the full (0,0) to (10,10)
    if let Some(area) = result {
        assert!(area < BigInt::from(121));
    }
}

#[test]
fn test_large_numbers() {
    // Test with larger coordinate values
    let input = "1000,2000
5000,2000
1000,8000
5000,8000";

    let result = find_largest_valid_rectangle(input);
    assert!(result.is_some());
    // Rectangle from (1000,2000) to (5000,8000)
    // Width: 4001, Height: 6001, Area: 24,010,001
    assert_eq!(result.unwrap(), BigInt::from(24010001));
}

#[test]
fn test_narrow_rectangle() {
    // Test a tall, narrow rectangle
    let input = "5,1
6,1
5,100
6,100";

    let result = find_largest_valid_rectangle(input);
    assert!(result.is_some());
    // Rectangle from (5,1) to (6,100)
    // Width: 2, Height: 100, Area: 200
    assert_eq!(result.unwrap(), BigInt::from(200));
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
    assert_eq!(result.unwrap(), BigInt::from(1));
}

#[test]
fn test_l_shaped_boundary() {
    // L-shaped points creating multiple valid rectangles
    let input = "0,0
10,0
0,10
3,10
3,7
0,7";

    let result = find_largest_valid_rectangle(input);
    assert!(result.is_some());
    // Should find some valid rectangle
    let area = result.unwrap();
    assert_eq!(area, BigInt::from(44));
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
    assert_eq!(area, BigInt::from(961));
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
    assert_eq!(area, BigInt::from(636));
}
