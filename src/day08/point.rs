#[derive(Debug, Clone, Copy)]
pub struct Point {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Point {
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Self { x, y, z }
  }

  pub fn distance(&self, other: &Point) -> f64 {
    let dx = self.x - other.x;
    let dy = self.y - other.y;
    let dz = self.z - other.z;
    f64::sqrt(dx*dx + dy*dy + dz*dz)
  }
}

impl PartialEq for Point {
  fn eq(&self, other: &Self) -> bool {
    self.x.to_bits() == other.x.to_bits()
      && self.y.to_bits() == other.y.to_bits()
      && self.z.to_bits() == other.z.to_bits()
  }
}

impl Eq for Point {}

impl std::hash::Hash for Point {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.x.to_bits().hash(state);
    self.y.to_bits().hash(state);
    self.z.to_bits().hash(state);
  }
}
