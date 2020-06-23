use super::custom_err::CustomErr;
use sdl2::mixer::{open_audio, Music, AUDIO_S16LSB, DEFAULT_CHANNELS};
use std::path::Path;

pub struct Audio<'a> {
  pub music: Music<'a>,
}

impl Audio<'_> {
  pub fn new(path: &str) -> Result<Self, CustomErr> {
    open_audio(48_000, AUDIO_S16LSB, DEFAULT_CHANNELS, 4_096)?;
    let music = Music::from_file(Path::new(path))?;
    Ok(Audio { music })
  }
}
