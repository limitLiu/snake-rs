use super::apple::Apple;
use super::point::Point;
use super::Direction;
use super::Direction::{Down, Left, Right, Up};
use super::{GREEN, HEIGHT, SIZE, WIDTH};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Player {
  color: Color,
  pub pos: Point,
  pub body: Vec<Point>,
}

impl Player {
  pub fn new() -> Self {
    Player {
      pos: Point::new(),
      color: GREEN,
      body: vec![Point::new()],
    }
  }

  pub fn reload(&mut self) {
    for i in (0..self.body.len()).rev() {
      if i == 0 {
        self.body[i].x += self.pos.x;
        self.body[i].y += self.pos.y;
      } else {
        self.body[i].x = self.body[i - 1].x;
        self.body[i].y = self.body[i - 1].y;
      }
    }
  }

  pub fn draw(&mut self, canvas: &mut WindowCanvas) {
    canvas.set_draw_color(self.color);
    for point in &self.body {
      canvas
        .fill_rect(Rect::new(point.x, point.y, SIZE, SIZE))
        .expect("Failed to draw");
    }
  }

  pub fn len_up(&mut self) {
    self.body.push(Point { ..*self.head() })
  }

  pub fn out_bounds(&self) -> bool {
    let head = self.head();
    head.x < 0
      || head.y < 0
      || head.x > (WIDTH - SIZE) as i32
      || head.y > (HEIGHT - SIZE) as i32
  }

  pub fn hit_self(&self) -> bool {
    for p in &self.body {
      return self.head().overlap(&p);
    }
    false
  }

  pub fn reset(&mut self) {
    self.body = vec![Point::new()];
    self.pos = Point::new();
  }

  pub fn eat(&self, apple: &Apple) -> bool {
    self.head().overlap(&apple.pos)
  }

  pub fn set_direction(&mut self, direction: Direction) {
    let pos = match direction {
      Up(p) => p,
      Right(p) => p,
      Down(p) => p,
      Left(p) => p,
    };
    self.pos = pos;
  }

  fn head(&self) -> &Point {
    &self.body[0]
  }
}
