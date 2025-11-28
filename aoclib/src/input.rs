use std::{fs::read_to_string, io::ErrorKind};

/// Return the input string corresponding to the current binary.
#[macro_export]
macro_rules! input {
    () => {
        aoclib::input::input_for(env!("CARGO_BIN_NAME"))
    };
}

/// Load the input file for the current puzzle.
pub fn input_for(puzzle: &str) -> String {
    let filename = format!("input/{puzzle}.txt");
    let mut path = filename;
    for _ in 0..5 {
        match read_to_string(&path) {
            Ok(s) => return s,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                path = "../".to_owned() + &path;
                continue;
            }
            Err(e) => panic!("{e:?}"),
        }
    }
    panic!("input not found in parents");
}
