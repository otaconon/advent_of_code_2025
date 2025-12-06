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

pub fn star(raw_data: String) -> String {
  let mut res = 0;
  let mut ops: Vec<char> = Vec::new();
  let mut nums: Vec<String> = Vec::new();
  let lines: Vec<&str> = raw_data.lines().rev().collect();
  let max_width = lines.iter().map(|s| s.len()).max().unwrap_or(0);

  for (i, line_iter) in lines.iter().enumerate() {
    let mut line: Vec<char> = line_iter.chars().collect();
    line.extend(vec![' '; max_width - line.len()]);

    if i == 0 {
      nums = vec![String::new(); line.len()];
      ops.extend(line);
      continue;
    }

    for (c, num) in std::iter::zip(line, &mut nums) {
      if c.is_alphanumeric() {
        num.push(c);
      }
    }
  }

  let mut prev_op = 'x';
  let mut partial = 0;
  for (op, num_str) in std::iter::zip(ops, nums) {
    let num: u128 = num_str
      .chars()
      .rev()
      .collect::<String>()
      .parse()
      .unwrap_or(0);
    if num == 0 {
      prev_op = 'x';
      continue;
    }

    if op == prev_op || op == ' ' {
      match prev_op {
        '*' => partial *= num,
        '+' => partial += num,
        _ => {}
      };
    } else {
      res += partial;
      prev_op = op;
      partial = num;
    }
  }
  res += partial;

  format!("{}", res)
}

pub fn star2(raw_data: String) -> String {
  let lines: Vec<Vec<char>> = raw_data
    .lines()
    .rev()
    .map(|line| line.chars().collect())
    .collect();

  let width = lines.iter().map(|s| s.len()).max().unwrap_or(0);
  let height = lines.len()-1;
  let mut right = width;
  let res = (0..width).rev().filter(|&col| lines[height][col] != ' ').fold(
    0, |acc, left| {
      let cols = (left..right)
        .map(|col| (0..height)
        .fold(0, |num, row| {
          match lines[row][col].to_digit(10) {
            Some(digit) => 10*num + digit as u128,
            None => num
          }
        }));
      right = left.saturating_sub(1);
      acc + match lines[height][left] {
        '+' => cols.sum(),
        '*' => cols.product(),
        _ => 0
      }
    }
  );

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
