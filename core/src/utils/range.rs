/*
    Appellation: boundary <module>
    Created At: 2025.08.31:15:10:27
    Contrib: @FL03
*/

/// returns a range, as a 2-tuple, around a given position within the bounds of some length and
/// with some radius
pub const fn get_range_around(position: usize, len: usize, radius: usize) -> (usize, usize) {
    let a = position.saturating_sub(radius);
    let b = if (position + radius) < len {
        position + radius
    } else {
        len - 1
    };
    (a, b)
}

#[doc(hidden)]
pub fn pretty_printer<T: core::fmt::Debug>(data: &[T], position: usize) -> String {
    let mut output = String::new();
    let (a, b) = get_range_around(position, data.len(), 10);
    // print out the tape with the head position highlighted
    for (idx, c) in (a..=b).zip(data[a..=b].iter()) {
        let cell = if position == idx || (idx == b && position == (idx + 1)) {
            format!("[[{c:?}]]")
        } else {
            format!("{c:?}")
        };
        output.push_str(&cell);
    }
    output
}

#[doc(hidden)]
pub fn printer<T: core::fmt::Display>(data: &[T], position: usize) -> String {
    let mut output = String::new();
    let (a, b) = get_range_around(position, data.len(), 10);
    // print out the tape with the head position highlighted
    for (idx, c) in (a..=b).zip(data[a..=b].iter()) {
        let cell = if position == idx || (idx == b && position == (idx + 1)) {
            format!("[[{c}]]")
        } else {
            format!("{c}")
        };
        output.push_str(&cell);
    }
    output
}
