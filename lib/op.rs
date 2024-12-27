/// Adds two unsigned 8-bit integers.
///
/// # Arguments
/// * `a` - First operand
/// * `b` - Second operand
///
/// # Returns
/// * The sum of `a` and `b`. If the result overflows, it wraps around (standard behavior for `u8`).
pub fn add(a: u8, b: u8) -> u8 {
    a.wrapping_add(b)
}

/// Subtracts two unsigned 8-bit integers.
///
/// # Arguments
/// * `a` - Minuend
/// * `b` - Subtrahend
///
/// # Returns
/// * `Ok(u8)` containing the result if no underflow occurs.
/// * `Err(String)` if underflow occurs.
pub fn sub(a: u8, b: u8) -> Result<u8, String> {
    a.checked_sub(b)
        .ok_or_else(|| "Subtraction resulted in underflow.".to_string())
}

/// Multiplies two unsigned 8-bit integers.
///
/// # Arguments
/// * `a` - First operand
/// * `b` - Second operand
///
/// # Returns
/// * The product of `a` and `b`. If the result overflows, it wraps around (standard behavior for `u8`).
pub fn mul(a: u8, b: u8) -> u8 {
    a.wrapping_mul(b)
}

/// Divides one unsigned 8-bit integer by another.
///
/// # Arguments
/// * `a` - Dividend
/// * `b` - Divisor
///
/// # Returns
/// * `Ok(u8)` containing the result of the division if `b` is nonzero.
/// * `Err(String)` if `b` is zero.
///
/// # Errors
/// Returns an error if division by zero is attempted.
pub fn div(a: u8, b: u8) -> Result<u8, String> {
    if b == 0 {
        Err("Division by zero is not allowed.".to_string())
    } else {
        Ok(a / b)
    }
}
