mod menu;
pub use menu::*;

mod player;
pub use player::*;

mod stat;
pub use stat::*;

#[allow(dead_code)]
pub struct Pos {
  pub x: i32,
  pub y: i32,
}

pub struct Title;
