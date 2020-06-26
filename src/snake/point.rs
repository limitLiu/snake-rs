use super::{SIZE, X_SCALE, Y_SCALE};
use rand::{thread_rng, Rng};
use std::ptr::eq;

#[derive(Eq, PartialEq, Debug)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl Point {
  pub fn new() -> Self {
    Point { x: 0, y: 0 }
  }

  pub fn overlap(&self, point: &Point) -> bool {
    return self.x == point.x && self.y == point.y && !eq(self, point);
  }

  pub fn random() -> Point {
    Point {
      x: (thread_rng().gen_range(0, X_SCALE) * SIZE) as i32,
      y: (thread_rng().gen_range(0, Y_SCALE) * SIZE) as i32,
    }
  }
}
