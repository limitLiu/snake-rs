use sdl2::video::WindowBuildError;
use sdl2::IntegerOrSdlError;
use std::error;

#[derive(Debug)]
pub enum CustomErr {
  SDLError(String),
}

impl std::fmt::Display for CustomErr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match *self {
      CustomErr::SDLError(ref cause) => write!(f, "{}", cause),
    }
  }
}

impl error::Error for CustomErr {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    Some(self)
  }
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
