pub mod intcode;
pub mod matrix;
pub mod permute;
mod point;

pub use matrix::Matrix;
pub use point::{point, Point};

use std::cmp::Ordering;

pub fn ordering_to_int(ord: Ordering) -> isize {
    match ord {
        Ordering::Less => -1,
        Ordering::Greater => 1,
        Ordering::Equal => 0,
    }
}
