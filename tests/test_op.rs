use bounce_shield::{add, div, mul, sub};

#[test]
fn test_add() {
    // Normal cases
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(100, 155), 255);

    // Overflow cases
    assert_eq!(add(255, 1), 0); // Wrap-around expected
    assert_eq!(add(128, 128), 0); // Wrap-around

    // Edge cases
    assert_eq!(add(0, 0), 0);
    assert_eq!(add(255, 0), 255);
}

#[test]
fn test_sub_unsigned() {
    // Normal cases
    assert_eq!(sub(10, 5), Ok(5));
    assert_eq!(sub(255, 200), Ok(55)); // can't stress test -1 numbers cuz rust won't even let you save your file, let alone compile 👌

    // Underflow cases
    assert_eq!(
        sub(5, 10),
        Err("Subtraction resulted in underflow.".to_string())
    );

    // Edge cases
    assert_eq!(sub(0, 0), Ok(0));
    assert_eq!(sub(255, 255), Ok(0));
}

#[test]
fn test_mul() {
    // Normal cases
    assert_eq!(mul(10, 5), 50);
    assert_eq!(mul(12, 12), 144);

    // Overflow cases
    assert_eq!(mul(128, 2), 0); // Wrap-around expected
    assert_eq!(mul(255, 2), 254); // Wrap-around

    // Edge cases
    assert_eq!(mul(0, 255), 0);
    assert_eq!(mul(255, 1), 255);
}

#[test]
fn test_div() {
    // Normal cases
    assert_eq!(div(10, 2), Ok(5));
    assert_eq!(div(255, 5), Ok(51));

    // Division by zero
    assert_eq!(
        div(10, 0),
        Err("Division by zero is not allowed.".to_string())
    );

    // Edge cases
    assert_eq!(div(0, 1), Ok(0));
    assert_eq!(div(255, 255), Ok(1));
}

#[test]
fn test_combinations() {
    // Try all combinations for small values to stress-test
    for a in 0..=255 {
        for b in 0..=255 {
            // Add
            let _ = add(a, b); // Wrapping, so no panic

            // Subtract
            let _ = sub(a, b); // Should return either Ok or Err, no panic

            // Multiply
            let _ = mul(a, b); // Wrapping, so no panic

            // Divide
            let _ = div(a, b); // Should return either Ok or Err, no panic
        }
    }
}
