/*
    Appellation: boundary <module>
    Created At: 2025.08.31:15:10:27
    Contrib: @FL03
*/

/// compute a range around the current position of the head using the given radius that is
/// within the bounds of the tape
pub fn get_range_around(position: usize, len: usize, radius: usize) -> (usize, usize) {
    // determine the initial position
    let start = if position < radius {
        0
    } else {
        position - radius
    };
    let end = if (position + radius) < len {
        position + radius
    } else {
        len - 1
    };
    (start, end)
}
