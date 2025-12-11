use std::{
  collections::{HashMap, HashSet, VecDeque},
  hash::Hash,
  u64,
};

fn count_paths<'a>(start: &'a str, end: &'a str, adj: &HashMap<&'a str, Vec<&'a str>>) -> u64 {
  let mut memo: HashMap<&'a str, u64> = HashMap::new();
  dfs(start, end, adj, &mut memo)
}

fn dfs<'a>(
  curr: &'a str,
  target: &'a str,
  adj: &HashMap<&'a str, Vec<&'a str>>,
  memo: &mut HashMap<&'a str, u64>,
) -> u64 {
  if curr == target {
    return 1;
  }

  if let Some(&count) = memo.get(curr) {
    return count;
  }

  let mut total_paths = 0;
  if let Some(neighbors) = adj.get(curr) {
    for &next_node in neighbors {
      total_paths += dfs(next_node, target, adj, memo);
    }
  }

  memo.insert(curr, total_paths);
  total_paths
}

pub fn star1(raw_data: String) -> String {
  let adj = parse_input(&raw_data);
  format!("{}", count_paths("you", "out", &adj))
}

fn calculate_path<'a>(path: Vec<&'a str>, adj: &HashMap<&'a str, Vec<&'a str>>) -> u64 {
  let mut res = 1;
  for node in 0..path.len() - 1 {
    res *= count_paths(path[node], path[node + 1], &adj);
  }
  res
}

pub fn star2(raw_data: String) -> String {
  let adj = parse_input(&raw_data);
  let mut ans = 0;

  ans += calculate_path(vec!["svr", "fft", "dac", "out"], &adj);
  ans += calculate_path(vec!["svr", "dac", "fft", "out"], &adj);

  format!("{}", ans)
}

fn parse_input(raw_data: &str) -> HashMap<&str, Vec<&str>> {
  let mut data: HashMap<&str, Vec<&str>> = HashMap::new();
  for line in raw_data.lines() {
    let s: Vec<&str> = line.trim().split(":").collect();
    let input = s[0].trim();
    let outputs: Vec<&str> = s[1].trim().split(" ").collect();
    data.insert(input, outputs);
  }

  data
}
