use point::Point;
use sdl2::pixels::Color;

pub mod apple;
pub mod audio;
pub mod err;
pub mod game;
pub mod player;
pub mod point;

const UP: Point = Point { x: 0, y: -1 };
const RIGHT: Point = Point { x: 1, y: 0 };
const DOWN: Point = Point { x: 0, y: 1 };
const LEFT: Point = Point { x: -1, y: 0 };

const SIZE: u32 = 32;

const X_SCALE: u32 = 30;
const Y_SCALE: u32 = 17;

const WIDTH: u32 = SIZE * X_SCALE;
const HEIGHT: u32 = SIZE * Y_SCALE;
const GRAY: Color = Color::RGB(100, 100, 100);
const RED: Color = Color::RGB(100, 10, 10);
const GREEN: Color = Color::RGB(10, 100, 10);

pub enum Direction {
  Up(Point),
  Right(Point),
  Down(Point),
  Left(Point),
}
