use super::point::Point;
use super::{RED, SIZE};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Apple {
  pub pos: Point,
  pub color: Color,
}

impl Apple {
  pub fn new() -> Self {
    Apple {
      color: RED,
      pos: Point::random(),
    }
  }

  pub fn draw(&self, canvas: &mut WindowCanvas) {
    canvas.set_draw_color(self.color);
    canvas
      .fill_rect(Rect::new(self.pos.x, self.pos.y, SIZE, SIZE))
      .expect("Failed to draw");
  }

  pub fn refresh(&mut self) {
    self.pos = Point::random();
  }
}
