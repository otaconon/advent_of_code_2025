use seq_macro::seq;
use std::env;
use std::fs;

seq!(N in 01..=12 {
  mod day~N;
});

pub fn read_input(day: u8) -> String {
  let path = format!("input/day{:02}.txt", day);
  fs::read_to_string(&path).unwrap_or_else(|_| panic!("Could not read input file: {}", path))
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let day_arg = args
    .get(1)
    .expect("Please provide a day number (e.g., '01')");

  seq!(N in 01..=12 {
    match day_arg.as_str() {
      #(
        stringify!(N) => day~N::solve(),
      )*
      _ => println!("Day {} not solved yet!", day_arg),
    }
  });
}
