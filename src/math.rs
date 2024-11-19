
pub struct Point {
  x: f64,
  y: f64
}

impl Point {
  pub fn new(x: f64, y: f64) -> Point {
    Point { x: x, y: y }
  }

  pub fn distance(&self, other: &Point) -> f64 {
    let x_squared = (other.x - self.x) * (other.x - self.x);
    let y_squared = (other.y - self.y) * (other.y - self.y);
    (x_squared + y_squared).sqrt()
  }
}
