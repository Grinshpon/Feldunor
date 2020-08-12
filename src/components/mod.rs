use bracket_lib::prelude::*;

#[allow(dead_code)]
pub struct Pos {
  pub x: usize,
  pub y: usize,
}

pub type Button = VirtualKeyCode;

pub struct Title;

#[derive(Debug)]
pub enum MenuOption {
  Start,
  Options,
}
pub struct Menu {
  pub options: Vec<MenuOption>,
  pub selected: usize,
}
