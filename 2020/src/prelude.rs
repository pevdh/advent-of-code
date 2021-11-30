pub use std::collections::{
    HashMap,
    HashSet
};

pub use std::fs::read_to_string;

use std::path::Path;

pub fn read_lines<P: AsRef<Path>>(path: P)  -> Vec<String>
{
    return std::fs::read_to_string(path).unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect()
}
