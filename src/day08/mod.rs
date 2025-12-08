use std::{
  cmp::Ordering,
  collections::{HashMap, HashSet},
};

mod point;
use point::Point;

mod dsu;
use dsu::Dsu;

pub fn star1(raw_data: String) -> String {
  let points = parse_input(&raw_data);
  let mut edges: Vec<(f64, Point, Point)> = Vec::with_capacity(points.len() * points.len() / 2);

  for i in 0..points.len() {
    for j in (i + 1)..points.len() {
      let u = points[i];
      let v = points[j];
      let dist = u.distance(&v);
      edges.push((dist, u, v));
    }
  }

  edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
  let mut adj = Dsu::new();
  for p in &points {
    adj.add(p);
  }

  let mut edges_count = 0;
  for (dist, u, v) in edges {
    if adj.find(u) != adj.find(v) {
      adj.union(&u, &v);
      edges_count += 1;
    }
  }

  let mut sizes: Vec<i32> = Vec::new();
  let mut visited_roots: HashSet<Point> = HashSet::new();

  for point in &points {
    let root = adj.find(*point);
    if !visited_roots.contains(&root) {
      sizes.push(adj.size[&root]);
      visited_roots.insert(root);
    }
  }

  sizes.sort_by(|a, b| b.cmp(a));
  let mut ans: i64 = 1;
  for i in 0..3 {
    if i >= sizes.len() {
      break;
    }
    ans *= sizes[i] as i64;
  }

  format!("{}", ans)
}

pub fn star2(raw_data: String) -> String {
  let points = parse_input(&raw_data);
  let mut edges: Vec<(f64, Point, Point)> = Vec::with_capacity(points.len() * points.len() / 2);

  for i in 0..points.len() {
    for j in (i + 1)..points.len() {
      let u = points[i];
      let v = points[j];
      let dist = u.distance(&v);
      edges.push((dist, u, v));
    }
  }

  edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
  let mut adj = Dsu::new();
  for p in &points {
    adj.add(p);
  }

  let mut ans = (Point::new(0.0,0.0,0.0), Point::new(0.0,0.0,0.0));
  for (dist, u, v) in edges {
    if adj.find(u) != adj.find(v) {
      adj.union(&u, &v);
      ans = (u, v);
    }
  }

  format!("{}", ans.0.x * ans.1.x)
}

fn parse_input(raw_data: &str) -> Vec<Point> {
  let mut data: Vec<Point> = Vec::new();
  for line in raw_data.lines() {
    let s: Vec<&str> = line.trim().split(",").collect();
    data.push(Point::new(
      s[0].parse().unwrap(),
      s[1].parse().unwrap(),
      s[2].parse().unwrap(),
    ));
  }

  data
}
