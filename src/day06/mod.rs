pub fn star1(raw_data: String) -> String {
  let data = parse_input(&raw_data);

  let mut res = 0;
  for (op, values) in data {
    res += match op {
      '*' => values.iter().product(),
      '+' => values.iter().sum(),
      _ => 0,
    };
  }

  format!("{}", res)
}

pub fn star2(raw_data: String) -> String {
  let mut res = 0;

  format!("{}", res)
}

fn parse_input(raw_data: &str) -> Vec<(char, Vec<u64>)> {
  let mut data: Vec<(char, Vec<u64>)> = Vec::new();
  for line in raw_data.lines() {
    let s: Vec<&str> = line.split_whitespace().collect();

    if data.is_empty() {
      for _ in 0..s.len() {
        data.push((' ', Vec::new()));
      }
    }

    for (i, x) in s.iter().enumerate() {
      let (op, nums) = &mut data[i];
      match x.chars().nth(0) {
        Some(y) if y.is_numeric() => nums.push(x.parse().unwrap()),
        Some(y) => *op = y,
        _ => continue,
      }
    }
  }

  data
}
