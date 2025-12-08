use std::collections::HashMap;

use crate::day08::point::Point;

pub struct Dsu {
  pub parent: HashMap<Point, Point>,
  pub size: HashMap<Point, i32>
}

impl Dsu {
  pub fn new() -> Self {
    Self {
      parent: HashMap::new(),
      size: HashMap::new(),
    }
  }

  pub fn add(&mut self, u: &Point) {
    self.parent.insert(*u, *u);
    self.size.insert(*u, 1);
  }

  pub fn find(&mut self, u: Point) -> Point {
    if self.parent[&u] == u {
      return u;
    }
    let mut root = u;
    let mut path = Vec::new();

    while self.parent[&root] != root {
      path.push(root);
      root = self.parent[&root];
    }

    for node in path {
      self.parent.insert(node, root);
    }

    root
  }

  pub fn union(&mut self, u: &Point, v: &Point) {
    let a = self.find(*u);
    let b = self.find(*v);
    if a != b {
      self.parent.insert(b, a);
      self.size.insert(a, self.size[&a] + self.size[&b]);
    }
  }
}
