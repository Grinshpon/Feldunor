mod menu;
pub use menu::*;

mod player;
pub use player::*;

#[allow(dead_code)]
pub struct Pos {
  pub x: usize,
  pub y: usize,
}

pub struct Title;
