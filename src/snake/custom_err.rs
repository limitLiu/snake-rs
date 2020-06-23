use sdl2::video::WindowBuildError;
use sdl2::IntegerOrSdlError;

#[derive(Debug)]
pub enum CustomErr {
  SDLError(String),
}

impl From<String> for CustomErr {
  fn from(err: String) -> Self {
    CustomErr::SDLError(err)
  }
}

impl From<WindowBuildError> for CustomErr {
  fn from(err: WindowBuildError) -> Self {
    CustomErr::SDLError(err.to_string())
  }
}

impl From<IntegerOrSdlError> for CustomErr {
  fn from(err: IntegerOrSdlError) -> Self {
    CustomErr::SDLError(err.to_string())
  }
}
