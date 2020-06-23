use super::SIZE;
use super::{HEIGHT, WIDTH};
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
    if !eq(self, point) {
      return self.x == point.x && self.y == point.y;
    }
    false
  }

  pub fn random() -> Point {
    Point {
      x: thread_rng().gen_range(0, (WIDTH - SIZE) as i32),
      y: thread_rng().gen_range(0, (HEIGHT - SIZE) as i32),
    }
  }
}
