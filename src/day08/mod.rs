use std::{cmp::Ordering, collections::{HashMap, HashSet}};

mod point;
use point::Point;

mod dsu;
use dsu::Dsu;

pub fn star1(raw_data: String) -> String {
  let points = parse_input(&raw_data);
  let mut closest: HashMap<Point, Vec<Point>> = HashMap::new();
  for a in &points {
    let mut neighbors = points.clone();

    neighbors.sort_by(|p1, p2| {
      let dist1 = a.distance(p1);
      let dist2 = a.distance(p2);

      dist1.partial_cmp(&dist2).unwrap_or(Ordering::Equal)
    });

    closest.insert(*a, neighbors);
  }

  let mut adj = Dsu::new();
  for (point, neighbors) in &mut closest {
    neighbors.remove(0);
    adj.add(&point);
  }

  for _ in 0..1000 {
    let mut least_distance = f64::MAX;
    let mut least_neighbors: Option<(Point, Point)> = None;
    for (point, neighbors) in &closest {
      if neighbors.is_empty() {
        continue;
      }
      let dist = point.distance(&neighbors[0]);
      if least_distance > dist {
        least_distance = dist;
        least_neighbors = Some((*point, neighbors[0]));
      }
    }
    if least_neighbors.is_none() {
      continue;
    }
    let (u, v) = least_neighbors.unwrap();
    adj.union(&u, &v);
    closest.get_mut(&u).unwrap().remove(0);
    closest.get_mut(&v).unwrap().remove(0);
  }

  let mut sizes: Vec<i32> = Vec::new();
  let mut visited: HashSet<Point> = HashSet::new();
  for point in &points {
    let p = adj.find(*point);
    if !visited.contains(&p) {
      sizes.push(adj.size[&p])
    }
    visited.insert(p);
  }

  sizes.sort_by(|a, b| b.cmp(a));
  let mut ans = 1;
  for i in 0..3 {
    if i >= sizes.len() {
      break;
    }
    ans *= sizes[i];
  }
  format!("{}", ans)
}

pub fn star2(raw_data: String) -> String {
  let points = parse_input(&raw_data);
  let mut closest: HashMap<Point, Vec<Point>> = HashMap::new();
  for a in &points {
    let mut neighbors = points.clone();

    neighbors.sort_by(|p1, p2| {
      let dist1 = a.distance(p1);
      let dist2 = a.distance(p2);

      dist1.partial_cmp(&dist2).unwrap_or(Ordering::Equal)
    });

    closest.insert(*a, neighbors);
  }

  let mut adj = Dsu::new();
  for (point, neighbors) in &mut closest {
    neighbors.remove(0);
    adj.add(&point);
  }

  let mut ans = (Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 0.0));
  loop {
    let mut least_distance = f64::MAX;
    let mut least_neighbors: Option<(Point, Point)> = None;
    for (point, neighbors) in &closest {
      if neighbors.is_empty() {
        continue;
      }
      let dist = point.distance(&neighbors[0]);
      if least_distance > dist {
        least_distance = dist;
        least_neighbors = Some((*point, neighbors[0]));
      }
    }
    if least_neighbors.is_none() {
      break;
    }
    let (u, v) = least_neighbors.unwrap();
    ans = (u, v);
    adj.union(&u, &v);
    let x = adj.find(u);
    if adj.size[&x] == points.len() as i32 {
      break;
    }
    closest.get_mut(&u).unwrap().remove(0);
    closest.get_mut(&v).unwrap().remove(0);
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
