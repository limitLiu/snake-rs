mod snake;

use snake::custom_err::CustomErr;
use snake::game::Game;
use snake::audio::Audio;

fn main() -> Result<(), CustomErr> {
  let mut game = Game::new("snake sdl2 rust")?;
  let audio = Audio::new("resources/audio/snake.ogg")?;
  audio.music.play(-1)?;
  while game.running {
    game.render();
    game.handle_events();
    std::thread::sleep(std::time::Duration::from_millis(10));
  }
  Ok(())
}
