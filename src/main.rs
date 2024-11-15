mod snake;

use snake::audio::Audio;
use snake::err::CustomErr;
use snake::game::Game;

fn main() -> Result<(), CustomErr> {
  let mut game = Game::new("snake sdl2 rust")?;
  let audio = Audio::new("resources/audio/snake.ogg")?;
  audio.music.play(-1)?;
  while game.running {
    game.render();
    game.handle_events();
  }
  Ok(())
}
