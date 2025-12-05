use std::fs;

pub fn read_input(day: u8) -> String {
  let path = format!("input/day{:02}.txt", day);
  fs::read_to_string(&path).unwrap_or_else(|_| panic!("Could not read input file: {}", path))
}
