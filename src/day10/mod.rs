use std::u32;

pub fn star1(raw_data: String) -> String {
  let data = parse_input(&raw_data);
  let mut ans = 0;

  for (target, schemes) in data {
    let n = schemes.len();
    let mut target_cnt = u32::MAX;
    for i in 0..(1 << n) {
      let mut xor = 0;
      let mut cnt = 0;
      for j in 0..n {
        if (i >> j) & 1 == 1 {
          xor ^= schemes[j];
          cnt += 1;
        }
      }
      if xor == target {
        target_cnt = u32::min(target_cnt, cnt);
      }
    }
    ans += target_cnt;
  }

  format!("{}", ans)
}

pub fn star2(raw_data: String) -> String {


  format!("{}", 0)
}

fn parse_input(raw_data: &str) -> Vec<(u32, Vec<u32>)> {
  let mut data: Vec<(u32, Vec<u32>)> = Vec::new();
  for line in raw_data.lines() {
    let s: Vec<&str> = line.trim().split(" ").collect();
    let mut target: u32 = 0;

    let target_str = s[0].strip_prefix("[").unwrap().strip_suffix("]").unwrap();
    for (i, c) in target_str.chars().into_iter().enumerate() {
      if c == '#' {
        target |= 1 << i;
      }
    }

    let mut schematics: Vec<u32> = Vec::new();
    let max_bit_index = s[0].len() as u32 - 1;
    for i in 1..s.len()-1 {
      let schematic = s[i].strip_prefix("(").unwrap().strip_suffix(")").unwrap();
      let schematic: Vec<&str> = schematic.split(",").collect();

      let mut scheme: u32 = 0;
      for idx_str in schematic {
        let idx = idx_str.parse::<u32>().unwrap();
        scheme |= 1 << idx;
      }
      schematics.push(scheme);
    }
    data.push((target, schematics));
  }

  data
}

