use std::{
  collections::{HashMap, HashSet, VecDeque},
  hash::Hash,
  u64,
};

pub fn star1(raw_data: String) -> String {
  let (shapes, trees) = parse_input(&raw_data);
  let mut cnt = 0;
  for (dims, indices) in &trees {
    let shape: u32 = indices.iter().sum::<u32>();
    if dims.0*dims.1 > shape*8 {
      cnt += 1
    }
  }
  format!("{}", cnt)
}

pub fn star2(raw_data: String) -> String {
  let adj = parse_input(&raw_data);
  let mut ans = 0;

  format!("{}", ans)
}

fn parse_input(raw_data: &str) -> ([[[u32; 3]; 3]; 6], Vec<((u32, u32), [u32; 6])>) {
  let mut shapes = [[[0u32; 3]; 3]; 6];
  let mut trees: Vec<((u32, u32), [u32; 6])> = Vec::new();

  let parts: Vec<&str> = raw_data.split("\r\n\r\n").collect();

  for (i, part) in parts.iter().enumerate() {
    if i < 6 {
      for (j, line) in part.lines().enumerate() {
        if j == 0 {
          continue;
        }
        if j - 1 >= 3 {
          break;
        }
        for (k, c) in line.trim().chars().enumerate() {
          if k >= 3 {
            break;
          }

          shapes[i][j - 1][k] = match c {
            '#' => 1,
            _ => 0,
          };
        }
      }
    } else {
      for line in part.lines() {
        if line.trim().is_empty() {
          continue;
        }

        let dims_shapes: Vec<&str> = line.split(':').collect();
        let dims: Vec<u32> = dims_shapes[0]
          .trim()
          .split('x')
          .map(|s| s.parse().unwrap_or(0))
          .collect();

        let shape_indices_vec: Vec<u32> = dims_shapes[1]
          .trim()
          .split_whitespace()
          .map(|s| s.parse().unwrap_or(0))
          .collect();

        let mut shape_indices = [0u32; 6];
        for (idx, &val) in shape_indices_vec.iter().enumerate() {
          shape_indices[idx] = val;
        }

        trees.push(((dims[0], dims[1]), shape_indices));
      }
    }
  }

  (shapes, trees)
}
