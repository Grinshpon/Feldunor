mod menu;
pub use menu::*;

mod player;
pub use player::*;

mod stat;
pub use stat::*;

mod visibility;
pub use visibility::*;

mod monster;
pub use monster::*;

mod render;
pub use render::*;

#[allow(dead_code)]
pub struct Pos {
  pub x: i32,
  pub y: i32,
}

pub struct Title;

pub struct Name(pub String);
