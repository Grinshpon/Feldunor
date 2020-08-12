mod menu;
pub use menu::*;

mod player;
pub use player::*;

#[allow(dead_code)]
pub struct Pos {
  pub x: i32,
  pub y: i32,
}

pub struct Title;
