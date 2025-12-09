fn field(a: &Point, b: &Point) -> i64 {
  ((f64::abs(a.x - b.x) + 1.0) * (f64::abs(a.y - b.y) + 1.0)) as i64
}

pub fn star1(raw_data: String) -> String {
  let points = parse_input(&raw_data);
  let mut ans = 1;
  for a in &points {
    for b in &points {
      ans = i64::max(ans, field(a, b));
    }
  }

  format!("{}", ans)
}

#[derive(Debug, Clone, Copy)]
struct Point {
  x: f64,
  y: f64,
}

#[derive(Debug, Clone)]
enum AxisAlignedLine {
  Vertical { x: f64, y_min: f64, y_max: f64 },
  Horizontal { y: f64, x_min: f64, x_max: f64 },
}

fn segment_cuts_rect(
  rect_x_min: f64,
  rect_x_max: f64,
  rect_y_min: f64,
  rect_y_max: f64,
  segment: &AxisAlignedLine,
) -> bool {
  match segment {
    AxisAlignedLine::Vertical { x, y_min, y_max } => {
      if *x > rect_x_min + f64::EPSILON && *x < rect_x_max - f64::EPSILON {
        let intersect_y_min = f64::max(*y_min, rect_y_min);
        let intersect_y_max = f64::min(*y_max, rect_y_max);
        if intersect_y_max > intersect_y_min {
          return true;
        }
      }
      false
    }
    AxisAlignedLine::Horizontal { y, x_min, x_max } => {
      if *y > rect_y_min + f64::EPSILON && *y < rect_y_max - f64::EPSILON {
        let intersect_x_min = f64::max(*x_min, rect_x_min);
        let intersect_x_max = f64::min(*x_max, rect_x_max);
        if intersect_x_max > intersect_x_min {
          return true;
        }
      }
      false
    }
  }
}

pub fn star2(raw_data: String) -> String {
  let points = parse_input(&raw_data);
  let n = points.len();
  let mut ans = 0;

  let mut segments: Vec<AxisAlignedLine> = Vec::new();
  for (i, a) in points.iter().enumerate() {
    let b = points[(i + 1) % n];
    if a.x == b.x {
      segments.push(AxisAlignedLine::Vertical {
        x: a.x,
        y_min: f64::min(a.y, b.y),
        y_max: f64::max(a.y, b.y),
      })
    }
    if a.y == b.y {
      segments.push(AxisAlignedLine::Horizontal {
        y: a.y,
        x_min: f64::min(a.x, b.x),
        x_max: f64::max(a.x, b.x),
      })
    }
  }

  for a in &points {
    for b in &points {
      let mut intersects = false;
      for segment in &segments {
        let x_min = f64::min(a.x, b.x);
        let x_max = f64::max(a.x, b.x);
        let y_min = f64::min(a.y, b.y);
        let y_max = f64::max(a.y, b.y);
        intersects |= segment_cuts_rect(x_min, x_max, y_min, y_max, segment);
      }
      if !intersects {
        ans = i64::max(ans, field(a, b));
      }
    }
  }

  format!("{}", ans)
}

fn parse_input(raw_data: &str) -> Vec<Point> {
  let mut data: Vec<Point> = Vec::new();
  for line in raw_data.lines() {
    let s: Vec<&str> = line.trim().split(",").collect();
    data.push(Point {
      x: s[0].parse().unwrap(),
      y: s[1].parse().unwrap(),
    });
  }

  data
}
