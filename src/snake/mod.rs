use point::Point;
use sdl2::pixels::Color;

pub mod apple;
pub mod audio;
pub mod custom_err;
pub mod game;
pub mod player;
pub mod point;

const UP: Point = Point { x: 0, y: -1 };
const RIGHT: Point = Point { x: 1, y: 0 };
const DOWN: Point = Point { x: 0, y: 1 };
const LEFT: Point = Point { x: -1, y: 0 };

const SIZE: u32 = 32;

const WIDTH: u32 = SIZE * 30;
const HEIGHT: u32 = SIZE * 17;
const GRAY: Color = Color::RGB(100, 100, 100);
const RED: Color = Color::RGB(100, 10, 10);
const GREEN: Color = Color::RGB(10, 100, 10);

pub enum Direction {
  Up(Point),
  Right(Point),
  Down(Point),
  Left(Point),
}
