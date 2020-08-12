use bracket_lib::prelude::*;
use shipyard::{IntoIter, ViewMut};
use crate::event::*;

#[allow(dead_code)]
pub struct Pos {
  pub x: usize,
  pub y: usize,
}

//pub type Button = VirtualKeyCode;

pub struct Title;

#[derive(Debug)]
pub enum MenuOption {
  Start,
  Options,
  Quit,
}
pub struct Menu {
  pub options: Vec<MenuOption>,
  pub selected: usize,
}

pub fn menu_system((event, enter): (BEvent, &mut bool), mut menus: ViewMut<Menu>) {
  if let BEvent::KeyboardInput {key, scan_code, pressed} = event {
    for menu in (&mut menus).iter() {
      if pressed {
        use crate::VirtualKeyCode::*;
        match &key {
          K | Up => {if menu.selected > 0 {menu.selected -= 1;}},
          J | Down => {if menu.selected < menu.options.len()-1 {menu.selected += 1;}},
          Space | Return => {*enter = true},
          _ => {},
        }
      }
    }
  }
}
