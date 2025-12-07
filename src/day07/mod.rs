use std::collections::{HashMap, HashSet};

pub fn star1(raw_data: String) -> String {
  let (start, grid) = parse_input(&raw_data);
  let mut beams: HashSet<usize> = HashSet::from([start.1]);
  let mut splits = 0;

  for row in grid.iter() {
    let mut new_beams = beams.clone();
    for beam in beams.iter() {
      if row[*beam] {
        new_beams.insert(*beam - 1);
        new_beams.insert(*beam + 1);
        new_beams.remove(beam);
        splits += 1;
      }
    }
    beams = new_beams;
  }

  format!("{}", splits)
}

pub fn star2(raw_data: String) -> String {
  let (start, grid) = parse_input(&raw_data);
  let mut beams: Vec<u64> = vec![1; grid[0].len()];

  for row in grid.iter().rev() {
    let mut new_beams = beams.clone();
    for (beam, cnt) in beams.iter().enumerate() {
      if row[beam] {
        new_beams[beam] -= cnt;
      }
      if beam < row.len()-1 && row[beam+1] {
        new_beams[beam+1] += cnt;
      }
      if beam > 0 && row[beam-1] {
        new_beams[beam-1] += cnt;
      }
    }
    beams = new_beams;
  }

  let mut res = 0;
  for (beam, cnt) in beams.iter().enumerate() {
    if beam == start.1 {
      res += cnt;
    }
  } 

  format!("{}", res)
}

fn parse_input(raw_data: &str) -> ((usize, usize), Vec<Vec<bool>>) {
  let mut data: Vec<Vec<bool>> = Vec::new();
  let mut start = (0, 0);
  for (row, line) in raw_data.lines().enumerate() {
    let s: Vec<char> = line.trim().chars().collect();
    data.push(vec![false; s.len()]);
    for (col, (c, x)) in std::iter::zip(s, data.last_mut().unwrap()).enumerate() {
      match c {
        '^' => *x = true,
        '.' => *x = false,
        'S' => start = (row, col),
        _ => {}
      }
    }
  }

  (start, data)
}

