//! Integers modulo some base

pub fn add_usize_mod(a: usize, b: usize, base: usize) -> usize {
    (a + b) % base
}

pub fn add_isize_mod(a: usize, b: isize, base: usize) -> usize {
    (a + (base as isize + b % base as isize) as usize) % base
}

pub fn sub_usize_mod(a: usize, b: usize, base: usize) -> usize {
    (a + base - (b % base)) % base
}
