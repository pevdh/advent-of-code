pub use std::collections::{
    HashMap,
    HashSet
};

pub fn read_lines(day: u32)  -> Vec<String>
{
    let path = format!("input/day_{}.txt", day);

    return std::fs::read_to_string(path).unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect()
}

pub fn read_integers(day: u32)  -> Vec<i64>
{
    let path = format!("input/day_{}.txt", day);

    return std::fs::read_to_string(path).unwrap()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect()
}
