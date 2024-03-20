use std::ffi::CString;

/// Converts a string to a mutable pointer to a C-style string.
///
/// Writes to a pointer of chars that can be written to a C pointer.
pub fn str_to_mut_i8_ptr(s: &str) -> *mut i8 {
    let cs = CString::new(s).unwrap();
    let mut cv: Vec<u8> = cs.into_bytes_with_nul();
    cv.as_mut_ptr() as *mut i8
}

/// Calculates the total energy as 1 over the total number of orders defined.
pub fn get_total_power(starting_order: f64, ending_order: f64) -> f64 {
    1.0 / ((ending_order - starting_order).abs() + 1.0)
}
