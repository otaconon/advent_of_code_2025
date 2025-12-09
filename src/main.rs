use seq_macro::seq;
use std::env;
use std::fs;

seq!(N in 06..=09 {
  mod day~N;
});

pub fn read_input(day: u8, file_name: &str) -> String {
  let path = format!("input/day{:02}/{}", day, file_name);
  let absolute_path = std::path::absolute(&path).unwrap();
  fs::read_to_string(&path)
    .unwrap_or_else(|_| panic!("Could not read input file: {}", absolute_path.display()))
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let day_arg = args
    .get(1)
    .expect("Please provide a day number (e.g., '01')");

  seq!(N in 06..=09 {
    match day_arg.as_str() {
      #(
        stringify!(N) => {
          println!("===== Example =====");
          println!("star 1: {}", day~N::star1(read_input(N, "example.in")));
          println!("star 2: {}", day~N::star2(read_input(N, "example.in")));
          println!("===== Input =====");
          println!("star 1: {}", day~N::star1(read_input(N, "input.in")));
          println!("star 2: {}", day~N::star2(read_input(N, "input.in")));
        }
      )*
      _ => println!("Day {} not solved yet!", day_arg),
    }
  });
}
