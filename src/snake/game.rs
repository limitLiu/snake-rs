use super::apple::Apple;
use super::custom_err::CustomErr;
use super::player::Player;
use super::Direction::*;
use super::{DOWN, LEFT, RIGHT, UP};
use super::{GRAY, HEIGHT, WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

pub struct Game {
  pub canvas: WindowCanvas,
  pub running: bool,
  pub events: EventPump,
  pub snake: Player,
  pub apple: Apple,
}

impl Game {
  pub fn new(title: &str) -> Result<Self, CustomErr> {
    let sdl_context = sdl2::init()?;
    let vs = sdl_context.video()?;
    let window = vs
      .window(title, WIDTH as u32, HEIGHT as u32)
      .position_centered()
      .opengl()
      .resizable()
      .build()?;

    let events = sdl_context.event_pump()?;
    let canvas = window.into_canvas().accelerated().present_vsync().build()?;
    let running = true;
    Ok(Game {
      canvas,
      running,
      events,
      snake: Player::new(),
      apple: Apple::new(),
    })
  }

  pub fn handle_events(&mut self) {
    let Game {
      events, running, ..
    } = self;
    for event in events.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => {
          *running = false;
        }
        Event::KeyDown {
          keycode: Some(Keycode::Up),
          ..
        } => {
          self.snake.set_direction(Up(UP));
        }
        Event::KeyDown {
          keycode: Some(Keycode::Right),
          ..
        } => {
          self.snake.set_direction(Right(RIGHT));
        }
        Event::KeyDown {
          keycode: Some(Keycode::Down),
          ..
        } => {
          self.snake.set_direction(Down(DOWN));
        }
        Event::KeyDown {
          keycode: Some(Keycode::Left),
          ..
        } => {
          self.snake.set_direction(Left(LEFT));
        }
        _ => {}
      }
    }
  }

  pub fn render(&mut self) {
    let Game { canvas, .. } = self;
    canvas.set_draw_color(GRAY);
    canvas.clear();
    self.apple.draw(canvas);
    self.snake.draw(canvas);
    canvas.present();

    if self.snake.eat(&self.apple) {
      self.apple.refresh();
      self.snake.len_up();
    }

    self.snake.reload();

    if self.snake.hit_self() || self.snake.out_bounds() {
      self.apple.refresh();
      self.snake.reset();
    }
  }
}
