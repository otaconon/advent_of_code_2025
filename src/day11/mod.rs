use std::{collections::{HashMap, HashSet}, hash::Hash, u32};

pub fn star1(raw_data: String) -> String {
  let adj = parse_input(&raw_data);
  let mut ans = 0;

  let mut st: Vec<&str> = Vec::new();
  let vis: HashSet<&str> = HashSet::new();
  let mut cnt: HashMap<&str, u32> = HashMap::new();
  st.push("you");
  while !st.is_empty() {
    let u = st.pop().unwrap();
    for v in &adj[u] {
      if !vis.contains(v) {
        st.push(v);
      }
      *cnt.entry(v).or_insert(0) += 1;
    }
  }

  format!("{}", cnt["out"])
}

pub fn star2(raw_data: String) -> String {


  format!("{}", 0)
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


